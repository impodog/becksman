use crate::prelude::*;
use becks_match::*;

pub fn acquire_round(login: &Login, round: Id, required: bool) -> Option<becks_match::Round> {
    login
        .db()
        .query_row(
            indoc! {"
        SELECT (left_win)
        FROM round
        WHERE id = (:id)
    "},
            rusqlite::named_params! {
                ":id": round.to_prim()
            },
            |row| {
                Ok(Round {
                    left_win: row.get(0)?,
                })
            },
        )
        .inspect_err(|err| {
            if required {
                error!("When acquiring required round {:?}, {}", round, err);
            }
        })
        .ok()
}

pub fn acquire_match(login: &Login, mat: Id, required: bool) -> Option<becks_match::Match> {
    login
        .db()
        .query_row(
            indoc! {"
            SELECT (left, right, round_worth, rounds, notes)
            FROM match
            WHERE id = (:id)
        "},
            rusqlite::named_params! {
                ":id": mat.to_prim(),
            },
            |row| {
                let rounds_str: String = row.get("rounds")?;
                let mut rounds = Vec::new();
                for round in rounds_str.split_whitespace() {
                    match round.parse::<u32>() {
                        Ok(round) => {
                            if let Some(round) =
                                acquire_round(login, Id::from_prim(round), required)
                            {
                                rounds.push(round)
                            }
                        }
                        Err(err) => {
                            error!(
                                "When acquiring round, value {:?} is not an id, {}",
                                round, err
                            );
                        }
                    }
                }
                Ok(Match {
                    total_rounds: rounds.len(),
                    left: Id::from_prim(row.get("left")?),
                    right: Id::from_prim(row.get("right")?),
                    round_worth: row.get("round_worth")?,
                    rounds,
                    quit: Quit::try_from(row.get::<_, u8>("quit")?).unwrap_or_else(|err| {
                        error!("When acquiring quit field in match, {}", err);
                        Default::default()
                    }),
                    notes: row.get("notes")?,
                })
            },
        )
        .inspect_err(|err| {
            if required {
                error!("When acquiring match, {}", err);
            }
        })
        .ok()
}
