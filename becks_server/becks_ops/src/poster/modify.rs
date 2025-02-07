use crate::prelude::*;

/// Lazy-modifies the poster information in the database, returns if modification is successful
pub fn modify_poster(login: &Login, poster: Id, value: &str) -> bool {
    login
        .db()
        .execute(
            indoc! {"
                UPDATE poster
                SET modified = TRUE,
                value = (:value)
                WHERE id = (:id)
            "},
            rusqlite::named_params! {
                ":value": value,
                ":id": poster.to_prim(),
            },
        )
        .inspect_err(|err| {
            error!("When modifying poster value, {}", err);
        })
        .is_ok_and(|modifies| modifies > 0)
}
