use crate::prelude::*;
use becks_convey::poster::acquire::*;

#[get("/acquire")]
pub(super) async fn acquire_poster(req: web::Json<AcquireRequest>, db: DbData) -> HttpResponse {
    debug!("Attempt to acquire poster {:?}", req.poster);
    let login = extract_login!(db, &req.token);
    if let Some(poster) = becks_ops::poster::acquire_poster(login.as_ref(), req.poster) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::json())
            .json(AcquireResponse { poster })
    } else {
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("unable to acquire desired poster")
    }
}
