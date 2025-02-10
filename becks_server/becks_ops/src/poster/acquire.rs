use crate::prelude::*;
use becks_poster::*;

pub fn acquire_poster(login: &Login, poster: Id) -> Option<Poster> {
    let db = login.db();
    if let Ok((value, images, timestamp)) = db
        .query_row(
            indoc! {"
                SELECT value, images, timestamp FROM poster
                WHERE id = (:id)
            "},
            rusqlite::named_params! {
                ":id": poster.to_prim(),
            },
            |row| {
                let value = row.get::<_, String>("value")?;
                let images = row.get::<_, String>("images")?;
                let timestamp = row.get::<_, u64>("timestamp")?;
                Ok((value, images, timestamp))
            },
        )
        .inspect_err(|err| {
            warn!("When querying for poster {:?}, {}", poster, err);
        })
    {
        let images = images
            .split('?')
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_owned())
                }
            })
            .collect();
        Some(Poster {
            value,
            images,
            timestamp,
        })
    } else {
        None
    }
}
