use crate::prelude::*;
use becks_match::*;

#[derive(Debug, Error)]
pub enum CreateMatchError {
    #[error("match is incomplete")]
    Incomplete,
}

fn create_match_id(login: &Login) -> Id {
    loop {
        let id = Id::rand();
        if login
            .db()
            .query_row(
                "SELECT id FROM match WHERE id = (:id)",
                rusqlite::named_params! {":id": id.to_prim()},
                |row| row.get::<_, u32>(0),
            )
            .is_err()
        {
            break id;
        }
    }
}

fn create_round_id(login: &Login) -> Id {
    loop {
        let id = Id::rand();
        if login
            .db()
            .query_row(
                "SELECT id FROM round WHERE id = (:id)",
                rusqlite::named_params! {":id": id.to_prim()},
                |row| row.get::<_, u32>(0),
            )
            .is_err()
        {
            break id;
        }
    }
}

pub fn create_round(login: &Login, round: &Round) -> Result<Id, CreateMatchError> {
    let id = create_round_id(login);
    login
        .db()
        .execute(
            indoc! {"
            INSERT INTO round
            (id, left_win)
            VALUES ((:id), (:left_win))
        "},
            rusqlite::named_params! {
                ":id": id.to_prim(),
                ":left_win": round.left_win,
            },
        )
        .inspect_err(|err| {
            error!("When creating a round, {}", err);
        })
        .ok();
    Ok(id)
}

pub fn create_match(login: &Login, mat: &Match) -> Result<Id, CreateMatchError> {
    if mat.total_rounds as usize != mat.rounds.len() {
        return Err(CreateMatchError::Incomplete);
    }
    let id = create_match_id(login);
    let mut rounds = String::new();
    let len = mat.rounds.len();
    for (index, round) in mat.rounds.iter().enumerate() {
        let round = create_round(login, round)?;
        rounds.push_str(&round.to_prim().to_string());
        if index + 1 != len {
            rounds.push(' ');
        }
    }
    login
        .db()
        .execute(
            indoc! {"
            INSERT INTO match
            (id, left, right, round_worth, rounds, notes)
            VALUES ((:id), (:left), (:right), (:round_worth), (:rounds), (:notes))
        "},
            rusqlite::named_params! {
                ":id": id.to_prim(),
                ":left": mat.left.to_prim(),
                ":right": mat.right.to_prim(),
                ":round_worth": mat.round_worth,
                ":rounds": rounds,
                ":notes": mat.notes,
            },
        )
        .inspect_err(|err| {
            error!("When creating match, {}", err);
        })
        .ok();
    Ok(id)
}
