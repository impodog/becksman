use crate::prelude::*;

pub fn delete_crew(login: &Login, id: CrewId) -> bool {
    let db = login.db();
    db.execute(
        indoc! {"DELETE FROM crew WHERE id = (:id)"},
        rusqlite::named_params! {":id": id.to_prim()},
    )
    .inspect_err(|err| {
        error!("When deleting id {:?}, {}", id, err);
    })
    .is_ok_and(|modified| modified > 0)
}
