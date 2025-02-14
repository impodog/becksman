use crate::prelude::*;
use becks_convey::mat::query::*;
use rusqlite::ToSql;

/// Returns a list of matching matches, if any
pub fn query(login: &Login, query: &QueryRequest) -> Vec<Id> {
    let mut sql = String::from("SELECT id FROM match");
    let mut store = Vec::new();
    let query_len = query.len();
    if !query.is_empty() {
        sql.push_str(" WHERE ");
    }
    let mut contains_time = false;
    let mut position = 1;
    for (index, query) in query.iter().enumerate() {
        match query {
            QueryMatchBy::Note(value) => {
                store.push(box_sql(format!("*{}*", value)));
                sql.push_str(&format!("notes GLOB ?{position}"));
            }
            QueryMatchBy::Player(player) => {
                store.push(box_sql(player.to_prim()));
                sql.push_str(&format!("(left = ?{position} OR right = ?{position})"));
            }
            QueryMatchBy::Time { mid, error } => {
                let left = mid.saturating_sub(*error);
                let right = mid.saturating_add(*error);
                let next_position = position + 1;
                sql.push_str(&format!(
                    "timestamp BETWEEN ?{position} AND ?{next_position}"
                ));
                store.push(box_sql(left));
                store.push(box_sql(right));
                position += 2;
                contains_time = true;
            }
        }
        if index + 1 != query_len {
            sql.push_str(" AND ");
        }
    }
    if !contains_time {
        sql.push_str(" ORDER BY timestamp DESC");
    }
    sql.push_str(&format!(" LIMIT {}", becks_db::CONFIG.db.mat_limit));
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
                            error!("When querying for rows in matches, {}", err);
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                })
                .unwrap_or_else(|err| {
                    error!("When querying for matches, {}", err);
                    Default::default()
                })
        }
        Err(err) => {
            error!("When preparing query for matches, {}", err);
            Default::default()
        }
    }
}
