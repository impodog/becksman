use crate::prelude::*;
use becks_convey::user::auth::*;
use std::sync::{LazyLock, Mutex};

#[derive(Error, Debug)]
enum LoginError {
    #[error("log-in information is wrong")]
    Wrong,
    #[error("username or password is illegal")]
    Illegal,
    #[error("server returns an unexpected status code")]
    Unexpected,
}

#[derive(Debug)]
pub struct Login {
    token: Token,
    pub end: Mutex<bool>,
}

impl Login {
    /// Attempts to create a user, then log-in with the user
    ///
    /// The first layer of Result is errors generated by creation,
    /// the second layer is errors generated by log-in
    pub async fn create(name: String, pass: String) -> Result<Result<Self>> {
        debug!("Attempt to create user name {}, pass {}", name, pass);
        let _response = CLIENT
            .post(server_url!("user/create"))
            .json(&CreateRequest {
                name: name.clone(),
                pass: pass.clone(),
            })
            .send()
            .await?
            .error_for_status()?;
        Ok(Self::log_in(name, pass).await)
    }

    /// Attempts to log in with given credentials
    pub async fn log_in(name: String, pass: String) -> Result<Self> {
        debug!("Attempt to log-in with name {}, pass {}", name, pass);
        let response = CLIENT
            .post(server_url!("user/login"))
            .json(&LoginRequest { name, pass })
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => {
                let response: LoginResponse = response.json().await?;
                Ok(Login {
                    token: response.token,
                    end: Mutex::new(false),
                })
            }
            StatusCode::BAD_REQUEST => Err(LoginError::Illegal.into()),
            StatusCode::UNAUTHORIZED => Err(LoginError::Wrong.into()),
            status => {
                error!("Server returns unexpected status code: {}", status);
                Err(LoginError::Unexpected.into())
            }
        }
    }

    /// Logs out of the server
    pub async fn log_out(&self) -> Result<()> {
        debug!("Attempt to log-out token {:?}", self.token());
        let _response = self
            .client()
            .post(server_url!("user/logout"))
            .json(&LogoutRequest {
                token: self.token(),
            })
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub fn client(&self) -> &reqwest::Client {
        &CLIENT
    }

    pub fn token(&self) -> Token {
        self.token
    }
}

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::new(10, 0))
        .build()
        .expect("client should succeed to build")
});
