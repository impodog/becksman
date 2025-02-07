use crate::prelude::*;
use becks_poster::*;

fn create_poster_id(login: &Login) -> Id {
    loop {
        let id = Id::rand();
        if login
            .db()
            .query_row(
                "SELECT id FROM poster WHERE id = (:id)",
                rusqlite::named_params! {":id": id.to_prim()},
                |row| row.get::<_, u32>(0),
            )
            .is_err()
        {
            break id;
        }
    }
}

/// Creates a poster in the data base, this always succeeds if everything is working properly
pub fn create_poster(login: &Login, value: &str) -> Id {
    let id = create_poster_id(login);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    login
        .db()
        .execute(
            indoc! {"
                INSERT INTO poster
                (id, value, modified, timestamp)
                VALUES ((:id), (:value), FALSE, (:timestamp))
            "},
            rusqlite::named_params! {
                ":id": id.to_prim(),
                ":value": value,
                ":timestamp": timestamp,
            },
        )
        .inspect_err(|err| {
            error!("When creating poster id {:?}, {}", id, err);
        })
        .ok();
    id
}
