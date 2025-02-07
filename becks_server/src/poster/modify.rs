use crate::prelude::*;
use becks_convey::poster::modify::*;

#[post("/modify")]
pub(super) async fn modify_poster(req: web::Json<ModifyRequest>, db: DbData) -> HttpResponse {
    debug!(
        "Attempt to modify poster {:?} content to {}",
        req.poster, req.value
    );
    let login = extract_login!(db, &req.token);
    if becks_ops::poster::modify_poster(login.as_ref(), req.poster, &req.value) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::plaintext())
            .body("poster is modified")
    } else {
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("unable to modify desired poster")
    }
}
