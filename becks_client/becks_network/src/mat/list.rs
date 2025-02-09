use super::data::*;
use crate::prelude::*;
use becks_convey::mat::*;
use becks_match::*;
use std::future::Future;
use std::ops::Deref;
use tokio::sync::RwLock;

impl crate::util::GetId for RwLock<MatchInfo> {
    async fn id(&self) -> Id {
        self.read().await.id()
    }
}

#[derive(Debug, Default)]
pub struct MatchList {
    list: Vec<RwLock<MatchInfo>>,
}

impl Deref for MatchList {
    type Target = Vec<RwLock<MatchInfo>>;
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl MatchList {
    /// Queries the match list and returns it
    pub async fn query(login: &Login, by: Vec<query::QueryMatchBy>) -> Result<Self> {
        let response = login
            .client()
            .get(server_url!("mat/query"))
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
                .map(|id| RwLock::new(MatchInfo::new(id)))
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

    /// Forces reload all matches from the server
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
