use crate::prelude::*;
use becks_convey::mat::query::*;
use rusqlite::ToSql;

fn box_sql(value: impl ToSql + 'static) -> Box<dyn ToSql> {
    Box::new(value)
}

/// Returns a list of matching matches, if any
pub fn query(login: &Login, query: &QueryRequest) -> Vec<Id> {
    let mut sql = String::from("SELECT id FROM match");
    let mut store = Vec::new();
    let query_len = query.len();
    if !query.is_empty() {
        sql.push_str(" WHERE ");
    }
    for (index, query) in query.iter().enumerate() {
        let index = index + 1;
        match query {
            QueryMatchBy::Note(value) => {
                store.push(box_sql(format!("*{}*", value)));
                sql.push_str(&format!("notes GLOB (:{index})"));
            }
            QueryMatchBy::Player(player) => {
                store.push(box_sql(player.to_prim()));
                sql.push_str(&format!("(left = (:{index}) OR right = (:{index}))"));
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
