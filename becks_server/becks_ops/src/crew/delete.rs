use crate::prelude::*;

pub fn delete_crew(login: &Login, crew: Id) -> bool {
    let db = login.db();
    db.execute(
        indoc! {"UPDATE crew SET deleted = TRUE WHERE id = (:id)"},
        rusqlite::named_params! {":id": crew.to_prim()},
    )
    .inspect_err(|err| {
        error!("When deleting crew {:?}, {}", crew, err);
    })
    .is_ok_and(|modified| modified > 0)
}
