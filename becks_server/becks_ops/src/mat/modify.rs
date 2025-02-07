use crate::prelude::*;

/// Returns true if modification is successful
pub fn modify_match_notes(login: &Login, mat: Id, notes: &str) -> bool {
    let db = login.db();
    db.execute(
        indoc! {"
            UPDATE match
            SET notes = (:notes)
            WHERE id = (:id)
        "},
        rusqlite::named_params! {
            ":notes": notes,
            ":id": mat.to_prim(),
        },
    )
    .is_ok_and(|modifies| modifies > 0)
}
