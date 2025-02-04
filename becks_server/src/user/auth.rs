use crate::prelude::*;
use becks_convey::user::auth::*;

#[post("/login")]
pub(super) async fn log_in(info: web::Json<LoginRequest>, db: DbData) -> HttpResponse {
    use becks_crew::check;
    info!("Log-in attempt: {} with password {}", info.name, info.pass);
    if check!(is_alnum info.name) && check!(is_alnum info.pass) {
        if let Some(token) = db.log_in(&info.name, &info.pass) {
            HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .json(LoginResponse { token })
        } else {
            error!("Log-in failed with given credentials");
            HttpResponse::Unauthorized()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to log-in with given credentials")
        }
    } else {
        error!("Given name or pass is not legal");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name or pass is not legal")
    }
}

#[post("/logout")]
pub(super) async fn log_out(info: web::Json<LogoutRequest>, db: DbData) -> HttpResponse {
    info!("Log-out attempt: token {:?}", info.token);
    if db.log_out(info.token) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::plaintext())
            .body("log-out done")
    } else {
        error!("Unable to find given token");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("token is not available")
    }
}

#[post("/create")]
pub(super) async fn create_user(info: web::Json<CreateRequest>, db: DbData) -> HttpResponse {
    info!("Create attempt: {} with password {}", info.name, info.pass);
    if check!(is_alnum info.name) && check!(is_alnum info.pass) {
        if db.create(&info.name, &info.pass) {
            HttpResponse::Ok()
                .content_type(http::header::ContentType::plaintext())
                .body("user created")
        } else {
            error!("Create user failed with given credentials");
            HttpResponse::BadRequest()
                .content_type(http::header::ContentType::plaintext())
                .body("given credentials resulted in conflicts")
        }
    } else {
        error!("Given name or pass is not legal");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name or pass is not legal")
    }
}
