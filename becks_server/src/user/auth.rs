use crate::prelude::*;
use becks_convey::user::auth::*;

#[post("/login")]
pub(super) async fn log_in(
    info: web::Json<LoginRequest>,
    db: web::Data<becks_db::Db>,
) -> HttpResponse {
    use becks_user::check;
    info!("Log-in attempt: {} with password {}", info.name, info.pass);
    if check!(is_alnum info.name) && check!(is_alnum info.pass) {
        if let Some(token) = db.log_in(&info.name, &info.pass) {
            info!("Login success with token {:?}", token);
            HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .json(LoginResponse { token })
        } else {
            info!("Log-in failed with given credentials");
            HttpResponse::Unauthorized()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to login with given credentials")
        }
    } else {
        error!("Given name or pass is not alphanumeric");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name or pass is not alphanumeric")
    }
}
