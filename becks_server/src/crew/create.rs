use crate::prelude::*;
use becks_convey::crew::create::*;

#[post("/create")]
pub(super) async fn create_crew(req: web::Json<CreateRequest>, db: DbData) -> HttpResponse {
    trace!("Call to create crew named {}", req.name);
    if check!(is_alnum req.name) {
        let login = extract_login!(db, &req.token);
        if let Some(id) = becks_ops::crew::create_crew(login.as_ref(), &req.name) {
            HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .json(CreateResponse { id })
        } else {
            warn!("Unable to create crew in the database");
            HttpResponse::InternalServerError()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to create crew in the database")
        }
    } else {
        warn!("Given name is not legal");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name is not legal")
    }
}
