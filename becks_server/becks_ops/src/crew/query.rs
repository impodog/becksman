use crate::prelude::*;
use rusqlite::ToSql;

pub trait Query {
    fn query(self, login: &Login) -> Vec<Id>;
}

pub struct QueryBy {
    pub by: Vec<CrewLocation>,
    pub fuzzy: bool,
}

macro_rules! handle_query_by {
    (iter $iter: ident, $login: ident) => {
        $iter.map(|ids| {
            ids.filter_map(|result| {
                result
                    .inspect_err(|err| {
                        error!(
                            "When querying an id(column not found?), {}",
                            err
                        );
                    })
                    .ok()
                    .map(Id::from_prim)
            })
            .collect::<Vec<Id>>()
        })
        .unwrap_or_else(|err| {
            error!("When querying all rows, {}",  err);
            Vec::new()
        })
    };
    ($sql: expr, $params: expr, $login: ident) => {{
        debug!("Querying columns with sql {}", $sql);
        match $login.db().prepare($sql) {
            Ok(mut stmt) => {
                let iter = stmt.query_map(
                    $params,
                    |row| row.get::<_, u32>(0),
                );
                handle_query_by!(iter iter, $login)
            }
            Err(err) => {
                error!("When querying the table, {}", err);
                Vec::new()
            }
        }
    }};
}

#[allow(dead_code)]
fn show_id_names(login: &Login) {
    let db = login.db();
    let mut stmt = db.prepare("SELECT id, name FROM crew").unwrap();
    let res = stmt
        .query_map([], |row| Ok((row.get_unwrap(0), row.get_unwrap(1))))
        .unwrap()
        .map(Result::unwrap)
        .collect::<Vec<(u64, String)>>();
    debug!("All ids and names: {:?}", res);
}

fn box_sql(value: impl ToSql + 'static) -> Box<dyn ToSql> {
    Box::new(value)
}

macro_rules! extend_query_sql {
    ($type: ty, $sql: ident, $params: ident, $index: expr, $value: expr) => {{
        $sql.push_str(&format!("{} = ?{}", <$type>::name(), $index));
        $params.push(box_sql($value.convert()));
    }};
}

impl Query for QueryBy {
    fn query(self, login: &Login) -> Vec<Id> {
        use crate::crew::Column;
        use CrewLocation as Loc;
        let len = self.by.len();
        let mut sql = String::from("SELECT id FROM crew");
        let mut params = Vec::new();

        // special: if the query is empty, there is no WHERE
        if !self.by.is_empty() {
            sql.push_str(" WHERE ");
        }
        for (index, loc) in self.by.into_iter().enumerate() {
            let index = index + 1;
            match loc {
                Loc::Name(name) => {
                    let name = if self.fuzzy {
                        sql.push_str(&format!("{} GLOB ?{}", String::name(), index));
                        format!("{}*", name)
                    } else {
                        sql.push_str(&format!("{} = ?{}", String::name(), index));
                        name.convert()
                    };
                    params.push(box_sql(name));
                }
                Loc::Social(social) => extend_query_sql!(Social, sql, params, index, social),
                Loc::Score(score) => extend_query_sql!(Score, sql, params, index, score),
                Loc::Gender(gender) => extend_query_sql!(Gender, sql, params, index, gender),
                Loc::Clothes(clothes) => extend_query_sql!(Clothes, sql, params, index, clothes),
                Loc::Hand(hand) => extend_query_sql!(Hand, sql, params, index, hand),
                Loc::Hold(hold) => extend_query_sql!(Hold, sql, params, index, hold),
                Loc::Paddle(paddle) => extend_query_sql!(Paddle, sql, params, index, paddle),
                Loc::Red(red) => extend_query_sql!(RedRubber, sql, params, index, red),
                Loc::Black(black) => extend_query_sql!(BlackRubber, sql, params, index, black),
            }
            // index is increased by 1 earlier
            if index != len {
                sql.push_str(" AND ");
            }
        }
        let params_ref = params
            .iter()
            .map(|boxed| boxed.as_ref())
            .collect::<Vec<_>>();
        handle_query_by!(&sql, params_ref.as_slice(), login)
    }
}
