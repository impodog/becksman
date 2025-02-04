use crate::{extract_login, prelude::*};
use becks_convey::crew::create::*;

#[post("/create")]
pub(super) async fn create_crew(info: web::Json<CreateRequest>, db: DbData) -> HttpResponse {
    info!("Request to create crew named {}", info.name);
    if check!(is_alnum info.name) {
        let login = extract_login!(db, &info.token);
        if let Some(id) = becks_ops::crew::create_crew(login.as_ref(), &info.name) {
            HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .json(CreateResponse { id })
        } else {
            // FIXME: Initialize other fields also
            error!("Unable to create crew in the database");
            HttpResponse::InternalServerError()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to create crew in the database")
        }
    } else {
        error!("Given name is not legal");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name is not legal")
    }
}
