use crate::prelude::*;
use becks_convey::mat::modify::*;

#[post("/modify")]
pub(super) async fn modify_mat(req: web::Json<ModifyRequest>, db: DbData) -> HttpResponse {
    debug!(
        "Attempt to modify match {:?} notes to {}",
        req.mat, req.notes
    );
    let login = extract_login!(db, &req.token);
    if becks_ops::mat::modify_match_notes(login.as_ref(), req.mat, &req.notes) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::plaintext())
            .body("modification done")
    } else {
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("unable to modify given match")
    }
}
