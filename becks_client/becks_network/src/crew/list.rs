use super::data::*;
use crate::prelude::*;
use becks_convey::crew::*;
use becks_crew::*;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use tokio::sync::RwLock;

#[derive(Default)]
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
        let mut list = Vec::new();
        for crew in std::mem::take(&mut self.list) {
            let id = crew.read().await.id();
            list.push((crew, f(id).await?));
        }
        async {
            list.sort_unstable_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs));
        }
        .await;
        self.list = list.into_iter().map(|(crew, _)| crew).collect();

        Ok(())
    }

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
