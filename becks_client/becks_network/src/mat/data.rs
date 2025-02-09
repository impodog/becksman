use crate::prelude::*;
use becks_convey::mat::*;
use becks_match::*;

#[derive(Debug, Clone)]
pub struct MatchInfo {
    pub id: Id,
    pub data: Option<Match>,
}

impl MatchInfo {
    pub fn new(id: Id) -> Self {
        Self { id, data: None }
    }

    /// Uploads a new match to the server, returning the created match info
    pub async fn create(login: &Login, mat: Match) -> Result<Self> {
        let response = login
            .client()
            .post(server_url!("mat/create"))
            .json(&create::CreateRequest {
                token: login.token(),
                mat,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: create::CreateResponse = response.json().await?;
        Ok(Self {
            id: response.mat,
            data: None,
        })
    }

    /// Forces to reload match data from the server
    pub async fn reload(&mut self, login: &Login) -> Result<&mut Match> {
        let response = login
            .client()
            .get(server_url!("mat/acquire"))
            .json(&acquire::AcquireRequest {
                token: login.token(),
                mat: self.id,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: acquire::AcquireResponse = response.json().await?;
        Ok(self.data.insert(response.mat))
    }

    pub fn id(&self) -> Id {
        self.id
    }

    /// Loads match data if not previously loaded, then returns it
    pub async fn load(&mut self, login: &Login) -> Result<&mut Match> {
        if self.data.is_none() {
            self.reload(login).await?;
        }
        Ok(self
            .data
            .as_mut()
            .expect("match data should be loaded after check"))
    }

    /// Un-loads the poster data, so that any further operation must be loaded again
    pub fn unload(&mut self) {
        self.data = None;
    }

    pub async fn modify(&mut self, login: &Login, notes: String) -> Result<()> {
        let _response = login
            .client()
            .post(server_url!("mat/modify"))
            .json(&modify::ModifyRequest {
                token: login.token(),
                mat: self.id(),
                notes,
            })
            .send()
            .await?
            .error_for_status()?;
        self.unload();
        Ok(())
    }
}
