use crate::prelude::*;

pub trait Query {
    fn query(self, login: &Login) -> Vec<CrewId>;
}

pub struct QueryBy {
    pub loc: CrewLocation,
}

macro_rules! handle_query_by {
    ($type: ty, $value: ident, $login: ident) => {{
        debug!("Querying column {}", <$type>::name());
        match $login.db().prepare(&formatdoc! {"
            SELECT id FROM crew
            WHERE {column} = (:value)
            ",
            column = <$type>::name(),
        }) {
            Ok(mut stmt) => stmt
                .query_map(
                    rusqlite::named_params! {
                        ":value": $value.convert(),
                    },
                    |row| row.get::<_, u64>(0),
                )
                .map(|ids| {
                    ids.filter_map(|result| {
                        result
                            .inspect_err(|err| {
                                error!(
                                    "When querying an id by {}(column not found?), {}",
                                    <$type>::name(),
                                    err
                                );
                            })
                            .ok()
                            .map(CrewId::from_prim)
                    })
                    .collect::<Vec<CrewId>>()
                })
                .unwrap_or_else(|err| {
                    error!("When querying all rows by {}, {}", <$type>::name(), err);
                    Vec::new()
                }),
            Err(err) => {
                error!("When querying the table by {}, {}", <$type>::name(), err);
                Vec::new()
            }
        }
    }};
}

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

impl Query for QueryBy {
    fn query(self, login: &Login) -> Vec<CrewId> {
        use crate::crew::Column;
        use CrewLocation as Loc;
        match self.loc {
            Loc::Name(name) => handle_query_by!(String, name, login),
            Loc::Social(social) => handle_query_by!(Social, social, login),
            Loc::Gender(gender) => handle_query_by!(Gender, gender, login),
            Loc::Clothes(clothes) => handle_query_by!(Clothes, clothes, login),
            Loc::Hand(hand) => handle_query_by!(Hand, hand, login),
            Loc::Hold(hold) => handle_query_by!(Hold, hold, login),
            Loc::Paddle(paddle) => handle_query_by!(Paddle, paddle, login),
            Loc::Red(red) => handle_query_by!(RedRubber, red, login),
            Loc::Black(black) => handle_query_by!(BlackRubber, black, login),
        }
    }
}
