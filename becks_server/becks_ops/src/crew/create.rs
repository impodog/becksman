use crate::prelude::*;

/// Returns a new id if crew creation is successful
pub fn create_crew(login: &Login, name: &str) -> Option<CrewId> {
    let id = loop {
        let id = CrewId::rand();
        if login
            .db()
            .query_row("SELECT id FROM crew WHERE id = ?1", [id.to_prim()], |row| {
                row.get::<_, u64>(0)
            })
            .is_err()
        {
            break id;
        }
    };
    if login
        .db
        .lock()
        .unwrap()
        .execute(
            indoc! {"
                INSERT INTO crew (id, name)
                VALUES ((:id), (:name))
        "},
            rusqlite::named_params! {
                ":id": id.to_prim(),
                ":name": name,
            },
        )
        .is_ok()
    {
        Some(id)
    } else {
        None
    }
}
