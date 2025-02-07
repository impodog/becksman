use crate::prelude::*;
use becks_convey::crew::delete::*;

#[post("/delete")]
pub(super) async fn delete_crew(req: web::Json<DeleteRequest>, db: DbData) -> HttpResponse {
    debug!("Attempt to delete crew with id {:?}", req.crew);
    let login = extract_login!(db, &req.token);
    if becks_ops::crew::delete_crew(&login, req.crew) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::plaintext())
            .body("crew deleted")
    } else {
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("unable to delete crew")
    }
}
