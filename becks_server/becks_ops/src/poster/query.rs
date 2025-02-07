use crate::prelude::*;
use becks_convey::poster::query::*;

pub fn query(login: &Login, query: &QueryRequest) -> Vec<Id> {
    let mut sql = String::from("SELECT id FROM poster");
    let mut store = Vec::new();
    let query_len = query.len();
    if !query.is_empty() {
        sql.push_str(" WHERE ");
    }
    let mut position = 1usize;
    for (index, query) in query.iter().enumerate() {
        let index = index + 1;
        match query {
            QueryPosterBy::Content(content) => {
                store.push(box_sql(format!("*{}*", content)));
                sql.push_str(&format!("value GLOB ?{position}"));
                position += 1;
            }
            QueryPosterBy::Time { mid, error } => {
                let left = mid.saturating_sub(*error);
                let right = mid.saturating_add(*error);
                let next_position = position + 1;
                sql.push_str(&format!(
                    "timestamp BETWEEN ?{position} AND ?{next_position}"
                ));
                store.push(box_sql(left));
                store.push(box_sql(right));
                position += 2;
            }
        }
        if index != query_len {
            sql.push_str(" AND ");
        }
    }
    debug!("Querying the database with sql {}", sql);
    match login.db().prepare(&sql) {
        Ok(mut stmt) => {
            let params = store
                .iter()
                .map(|value| value.as_ref())
                .collect::<Vec<&dyn ToSql>>();
            stmt.query_map(params.as_slice(), |row| row.get::<_, u32>(0))
                .map(|iter| {
                    iter.filter_map(|value| match value {
                        Ok(value) => Some(Id::from_prim(value)),
                        Err(err) => {
                            error!("When querying for rows in posters, {}", err);
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                })
                .unwrap_or_else(|err| {
                    error!("When querying for posters, {}", err);
                    Default::default()
                })
        }
        Err(err) => {
            error!("When preparing query for poster, {}", err);
            Default::default()
        }
    }
}
