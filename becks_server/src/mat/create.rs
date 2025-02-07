use crate::prelude::*;
use becks_convey::mat::create::*;

#[post("/create")]
pub(super) async fn create_mat(req: web::Json<CreateRequest>, db: DbData) -> HttpResponse {
    #[cfg(debug_assertions)]
    debug!(
        "Attempt to create match {:?}",
        serde_json::to_string(&req.mat)
    );
    let login = extract_login!(db, &req.token);
    match becks_ops::mat::create_match(login.as_ref(), &req.mat) {
        Ok(mat) => {
            if becks_ops::mat::update_crew(login.as_ref(), &req.mat) {
                HttpResponse::Ok()
                    .content_type(http::header::ContentType::json())
                    .json(CreateResponse { mat })
            } else {
                HttpResponse::BadRequest()
                    .content_type(http::header::ContentType::plaintext())
                    .body("match created, but user modification failed")
            }
        }
        Err(err) => {
            warn!("Unable to create match because {}", err);
            HttpResponse::BadRequest()
                .content_type(http::header::ContentType::plaintext())
                .body(format!("{}", err))
        }
    }
}
