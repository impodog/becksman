use crate::prelude::*;
use becks_convey::poster::*;
use becks_poster::*;

#[derive(Clone)]
pub struct PosterInfo {
    pub id: Id,
    pub data: Option<Poster>,
}

impl PosterInfo {
    pub fn new(id: Id) -> Self {
        Self { id, data: None }
    }

    /// Uploads a new poster to the server, returning the created poster info
    ///
    /// Only the value data is required, and compilation will be done at the server-side
    pub async fn create(login: &Login, value: String) -> Result<Self> {
        let response = login
            .client()
            .post(server_url!("poster/create"))
            .json(&create::CreateRequest {
                token: login.token(),
                value,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: create::CreateResponse = response.json().await?;
        Ok(Self {
            id: response.poster,
            data: None,
        })
    }

    /// Forces to reload poster data from the server
    pub async fn reload(&mut self, login: &Login) -> Result<&mut Poster> {
        let response = login
            .client()
            .get(server_url!("poster/acquire"))
            .json(&acquire::AcquireRequest {
                token: login.token(),
                poster: self.id(),
            })
            .send()
            .await?
            .error_for_status()?;
        let response: acquire::AcquireResponse = response.json().await?;
        Ok(self.data.insert(response.poster))
    }

    pub fn id(&self) -> Id {
        self.id
    }

    /// Loads poster data if not previously loaded, then returns it
    pub async fn load(&mut self, login: &Login) -> Result<&mut Poster> {
        if self.data.is_none() {
            self.reload(login).await?;
        }
        Ok(self
            .data
            .as_mut()
            .expect("poster data should be loaded after check"))
    }

    /// Un-loads the poster data, so that any further operation must be loaded again
    pub fn unload(&mut self) {
        self.data = None;
    }

    pub async fn modify(&mut self, login: &Login, value: String) -> Result<()> {
        let _response = login
            .client()
            .post(server_url!("poster/modify"))
            .json(&modify::ModifyRequest {
                token: login.token(),
                poster: self.id(),
                value,
            })
            .send()
            .await?
            .error_for_status()?;
        self.unload();
        Ok(())
    }
}
