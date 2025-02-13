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
    let mut mat = req.mat.clone();
    if let Some((left_earn, right_earn)) = becks_ops::mat::update_crew(login.as_ref(), &mat) {
        mat.left_earn = left_earn;
        mat.right_earn = right_earn;
        match becks_ops::mat::create_match(login.as_ref(), &mat) {
            Ok(mat) => HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .json(CreateResponse { mat }),
            Err(err) => {
                warn!("Unable to create match because {}", err);
                HttpResponse::BadRequest()
                    .content_type(http::header::ContentType::plaintext())
                    .body(format!("{}", err))
            }
        }
    } else {
        error!("When creating match, unable to update crew score");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("unable to update score")
    }
}
