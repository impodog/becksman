use super::data::*;
use crate::prelude::*;
use becks_convey::crew::*;
use becks_crew::*;
use std::future::Future;
use std::ops::Deref;
use tokio::sync::RwLock;

impl crate::util::GetId for RwLock<CrewInfo> {
    async fn id(&self) -> Id {
        self.read().await.id()
    }
}

#[derive(Debug, Default)]
pub struct CrewList {
    list: Vec<RwLock<CrewInfo>>,
}

impl Deref for CrewList {
    type Target = Vec<RwLock<CrewInfo>>;
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl CrewList {
    /// Queries the crew list and returns the stored form
    pub async fn query(login: &Login, by: Vec<CrewLocation>) -> Result<Self> {
        let response = login
            .client()
            .get(server_url!("crew/query"))
            .json(&query::QueryByRequest {
                token: login.token(),
                by,
                fuzzy: true,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: query::QueryByResponse = response.json().await?;
        let result = Self {
            list: response
                .ids
                .into_iter()
                .map(|id| RwLock::new(CrewInfo::new(id)))
                .collect(),
        };
        Ok(result)
    }

    /// Asynchronously sorts the list by the value provided by the function
    pub async fn sort_by_value<F, R, T>(&mut self, f: F) -> Result<()>
    where
        F: Fn(Id) -> R,
        R: Future<Output = Result<T>>,
        T: Ord,
    {
        crate::util::sort_by_value(&mut self.list, f).await
    }

    /// Forces reload all crew from the server
    pub async fn reload(&self, login: &Login) -> Result<()> {
        let mut futures = Vec::new();
        for crew in self.list.iter() {
            futures.push(async { crew.write().await.reload(login).await.map(|_| ()) });
        }
        for future in futures.into_iter() {
            future.await?;
        }
        Ok(())
    }
}
