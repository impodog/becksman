use crate::prelude::*;

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

macro_rules! extend_query_sql {
    ($type: ty, $sql: ident, $params: ident, $index: expr, $value: expr) => {{
        $sql.push_str(&format!("{} = ?{}", <$type>::name(), $index));
        $params.push(box_sql($value.convert()));
        $index += 1;
    }};
}

fn no_produce_condition(loc: &CrewLocation) -> bool {
    matches!(loc, CrewLocation::Score(_))
}

fn not_deleted(loc: &CrewLocation) -> bool {
    !matches!(loc, CrewLocation::Deleted(_))
}

impl Query for QueryBy {
    fn query(mut self, login: &Login) -> Vec<Id> {
        use crate::crew::Column;
        use CrewLocation as Loc;
        let mut sql = String::from("SELECT id FROM crew");
        let mut params = Vec::new();

        if self.by.iter().all(not_deleted) {
            self.by.push(Loc::Deleted(false));
        }
        // special: if the query is empty, there is no WHERE
        if !self.by.is_empty() && (!self.fuzzy || !self.by.iter().all(no_produce_condition)) {
            sql.push_str(" WHERE ");
        }
        let mut fuzzy_score: Option<Score> = None;
        let mut pos: usize = 1;
        for loc in self.by.into_iter() {
            if pos != 1 && !no_produce_condition(&loc) {
                sql.push_str(" AND ");
            }
            match loc {
                Loc::Name(name) => {
                    let name = if self.fuzzy {
                        sql.push_str(&format!("{} GLOB ?{}", String::name(), pos));
                        format!("*{}*", name)
                    } else {
                        sql.push_str(&format!("{} = ?{}", String::name(), pos));
                        name.convert()
                    };
                    pos += 1;
                    params.push(box_sql(name));
                }
                Loc::Social(social) => extend_query_sql!(Social, sql, params, pos, social),
                Loc::Score(score) => {
                    if self.fuzzy {
                        fuzzy_score = Some(score);
                    } else {
                        extend_query_sql!(Score, sql, params, pos, score)
                    }
                }
                Loc::Gender(gender) => extend_query_sql!(Gender, sql, params, pos, gender),
                Loc::Clothes(clothes) => extend_query_sql!(Clothes, sql, params, pos, clothes),
                Loc::Hand(hand) => extend_query_sql!(Hand, sql, params, pos, hand),
                Loc::Hold(hold) => extend_query_sql!(Hold, sql, params, pos, hold),
                Loc::Paddle(paddle) => extend_query_sql!(Paddle, sql, params, pos, paddle),
                Loc::Red(red) => extend_query_sql!(RedRubber, sql, params, pos, red),
                Loc::Black(black) => extend_query_sql!(BlackRubber, sql, params, pos, black),
                Loc::Deleted(deleted) => extend_query_sql!(bool, sql, params, pos, deleted),
                Loc::Beat(beat) => extend_query_sql!(Beat, sql, params, pos, beat),
            }
        }
        #[allow(unused_assignments)]
        if let Some(score) = fuzzy_score {
            sql.push_str(&format!(" ORDER BY ABS({} - ?{})", Score::name(), pos));
            params.push(box_sql(score.convert()));
            pos += 1;
        }
        let params_ref = params
            .iter()
            .map(|boxed| boxed.as_ref())
            .collect::<Vec<_>>();
        handle_query_by!(&sql, params_ref.as_slice(), login)
    }
}
