use crate::prelude::*;
use becks_convey::crew::*;
use becks_crew::*;

#[derive(Debug, Clone)]
pub struct CrewInfo {
    id: Id,
    data: Option<CrewData>,
}

impl CrewInfo {
    pub fn new(id: Id) -> Self {
        Self { id, data: None }
    }

    /// Uploads a new crew to the server, returning the created crew
    pub async fn create(login: &Login, name: String, social: Social) -> Result<Self> {
        let response = login
            .client()
            .post(server_url!("crew/create"))
            .json(&create::CreateRequest {
                token: login.token(),
                name,
                social,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: create::CreateResponse = response.json().await?;
        Ok(Self {
            id: response.crew,
            data: None,
        })
    }

    /// Deletes the crew and consumes the crew info
    pub async fn delete(self, login: &Login) -> Result<()> {
        let _response = login
            .client()
            .post(server_url!("crew/delete"))
            .json(&delete::DeleteRequest {
                token: login.token(),
                crew: self.id,
            })
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// Forces to reload user data from server
    pub async fn reload(&mut self, login: &Login) -> Result<&mut CrewData> {
        let response = login
            .client()
            .get(server_url!("crew/acquire"))
            .json(&modify::AcquireRequest {
                token: login.token(),
                crew: self.id,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: modify::AcquireResponse = response.json().await?;
        Ok(self.data.insert(response.crew))
    }

    pub fn id(&self) -> Id {
        self.id
    }

    /// Loads crew data if not previously loaded, then returns it
    pub async fn load(&mut self, login: &Login) -> Result<&mut CrewData> {
        if self.data.is_none() {
            self.reload(login).await?;
        }
        Ok(self
            .data
            .as_mut()
            .expect("crew data should be loaded after check"))
    }

    /// Un-loads the crew data, so that any further operation must be loaded again
    pub fn unload(&mut self) {
        self.data = None;
    }

    pub async fn modify(&mut self, login: &Login, loc: CrewLocation) -> Result<()> {
        let _response = login
            .client()
            .post(server_url!("crew/modify"))
            .json(&modify::ModifyRequest {
                token: login.token(),
                crew: self.id,
                loc,
            })
            .send()
            .await?
            .error_for_status()?;
        self.unload();
        Ok(())
    }
}
