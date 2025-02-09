use crate::prelude::*;
use becks_crew::Id;
use std::future::Future;

pub(crate) trait GetId {
    async fn id(&self) -> Id;
}

/// Asynchronously sorts the list by the value provided by the function
pub(crate) async fn sort_by_value<V, F, R, T>(src: &mut Vec<V>, f: F) -> Result<()>
where
    V: GetId,
    F: Fn(Id) -> R,
    R: Future<Output = Result<T>>,
    T: Ord,
{
    let mut list = Vec::new();
    for value in std::mem::take(src) {
        let id = value.id().await;
        list.push((value, f(id).await?));
    }
    async {
        list.sort_unstable_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs));
    }
    .await;
    *src = list.into_iter().map(|(crew, _)| crew).collect();

    Ok(())
}
