use crate::prelude::*;
use becks_poster::*;

fn compile_markdown(md: &str) -> String {
    debug!("Compiling markdown {}", md);
    let parser = pulldown_cmark::Parser::new(md);
    let mut result = String::new();
    pulldown_cmark::html::push_html(&mut result, parser);
    debug!("Result is {}", result);
    result
}

pub fn acquire_poster(login: &Login, poster: Id) -> Option<Poster> {
    let db = login.db();
    if let Ok((value, timestamp, compiled)) = db
        .query_row(
            indoc! {"
                SELECT value, compiled, modified, timestamp FROM poster
                WHERE id = (:id)
            "},
            rusqlite::named_params! {
                ":id": poster.to_prim(),
            },
            |row| {
                let value = row.get::<_, String>("value")?;
                let compiled = row.get::<_, String>("compiled").ok();
                let modified = row.get::<_, bool>("modified")?;
                let timestamp = row.get::<_, u64>("timestamp")?;
                if modified {
                    Ok((value, timestamp, None))
                } else {
                    Ok((value, timestamp, compiled))
                }
            },
        )
        .inspect_err(|err| {
            warn!("When querying for poster {:?}, {}", poster, err);
        })
    {
        let compiled = if let Some(compiled) = compiled {
            compiled
        } else {
            let compiled = compile_markdown(&value);
            db.execute(
                indoc! {"
                        UPDATE poster
                        SET compiled = (:compiled),
                            modified = FALSE
                        WHERE id = (:id)
                    "},
                rusqlite::named_params! {
                    ":compiled": &compiled,
                    ":id": poster.to_prim()
                },
            )
            .inspect_err(|err| {
                error!("When updating compile value of markdown, {}", err);
            })
            .ok();
            compiled
        };
        Some(Poster {
            value,
            compiled,
            timestamp,
        })
    } else {
        None
    }
}
