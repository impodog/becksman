use crate::prelude::*;
use becks_convey::poster::create::*;

#[post("/create")]
pub(super) async fn create_poster(req: web::Json<CreateRequest>, db: DbData) -> HttpResponse {
    debug!("Attempt to create poster with content {}", req.value);
    let login = extract_login!(db, &req.token);
    let poster =
        becks_ops::poster::create_poster(login.as_ref(), &req.value, req.images.as_slice());
    HttpResponse::Ok()
        .content_type(http::header::ContentType::json())
        .json(CreateResponse { poster })
}
