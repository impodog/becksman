use crate::prelude::*;
use becks_convey::mat::acquire::*;

#[get("/acquire")]
pub(super) async fn acquire_mat(req: web::Json<AcquireRequest>, db: DbData) -> HttpResponse {
    debug!(
        "Attempt to acquire with token {:?}, match id {:?}",
        req.token, req.mat
    );
    let login = extract_login!(db, &req.token);
    if let Some(mat) = becks_ops::mat::acquire_match(login.as_ref(), req.mat, true) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::json())
            .json(AcquireResponse { mat })
    } else {
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("unable to acquire given match id")
    }
}
