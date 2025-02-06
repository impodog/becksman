use crate::prelude::*;

/// Returns a new id if crew creation is successful
pub fn create_crew(login: &Login, name: &str, social: Social) -> Option<CrewId> {
    let id = loop {
        let id = CrewId::rand();
        if login
            .db()
            .query_row("SELECT id FROM crew WHERE id = ?1", [id.to_prim()], |row| {
                row.get::<_, u32>(0)
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
                INSERT INTO crew (id, name, social, score)
                VALUES ((:id), (:name), (:social), 0)
        "},
            rusqlite::named_params! {
                ":id": id.to_prim(),
                ":name": name,
                ":social": super::Column::convert(social),
            },
        )
        .inspect_err(|err| {
            error!(
                "When creating new crew with id {:?} and name {}, {}",
                id, name, err
            );
        })
        .is_ok_and(|modified| modified > 0)
    {
        Some(id)
    } else {
        None
    }
}
