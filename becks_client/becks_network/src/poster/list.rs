use super::data::*;
use crate::prelude::*;
use becks_convey::poster::*;
use becks_poster::*;
use std::future::Future;
use std::ops::Deref;
use tokio::sync::RwLock;

impl crate::util::GetId for RwLock<PosterInfo> {
    async fn id(&self) -> Id {
        self.read().await.id()
    }
}

#[derive(Default)]
pub struct PosterList {
    list: Vec<RwLock<PosterInfo>>,
}

impl Deref for PosterList {
    type Target = Vec<RwLock<PosterInfo>>;
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl PosterList {
    /// Queries the poster list and returns it
    pub async fn query(login: &Login, by: Vec<query::QueryPosterBy>) -> Result<Self> {
        let response = login
            .client()
            .get(server_url!("poster/query"))
            .json(&query::QueryRequest {
                token: login.token(),
                by,
            })
            .send()
            .await?
            .error_for_status()?;
        let response: query::QueryResponse = response.json().await?;
        let result = Self {
            list: response
                .ids
                .into_iter()
                .map(|id| RwLock::new(PosterInfo::new(id)))
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

    /// Forces reload all poster from the server
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
