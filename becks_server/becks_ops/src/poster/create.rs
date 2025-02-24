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
pub fn create_poster(login: &Login, value: &str, images: &[String]) -> Id {
    let id = create_poster_id(login);
    let mut images_str = String::new();
    for image in images {
        images_str.push_str(image);
        images_str.push('?');
    }
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    login
        .db()
        .execute(
            indoc! {"
                INSERT INTO poster
                (id, value, images, timestamp)
                VALUES ((:id), (:value), (:images), (:timestamp))
            "},
            rusqlite::named_params! {
                ":id": id.to_prim(),
                ":value": value,
                ":images": images_str,
                ":timestamp": timestamp,
            },
        )
        .inspect_err(|err| {
            error!("When creating poster id {:?}, {}", id, err);
        })
        .ok();
    id
}
