use crate::prelude::*;
use becks_convey::user::auth::*;

#[post("/login")]
pub(super) async fn log_in(req: web::Json<LoginRequest>, db: DbData) -> HttpResponse {
    use becks_crew::check;
    debug!("Log-in attempt: {} with password {}", req.name, req.pass);
    if check!(is_alnum req.name) && check!(is_alnum req.pass) {
        if let Some(token) = db.log_in(&req.name, &req.pass) {
            HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .json(LoginResponse { token })
        } else {
            warn!("Log-in failed with given credentials");
            HttpResponse::Unauthorized()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to log-in with given credentials")
        }
    } else {
        warn!("Given name or pass is not legal");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name or pass is not legal")
    }
}

#[post("/logout")]
pub(super) async fn log_out(req: web::Json<LogoutRequest>, db: DbData) -> HttpResponse {
    debug!("Log-out attempt: token {:?}", req.token);
    if db.log_out(req.token) {
        HttpResponse::Ok()
            .content_type(http::header::ContentType::plaintext())
            .body("log-out done")
    } else {
        warn!("Unable to find given token");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("token is not available")
    }
}

#[post("/create")]
pub(super) async fn create_user(req: web::Json<CreateRequest>, db: DbData) -> HttpResponse {
    debug!("Create attempt: {} with password {}", req.name, req.pass);
    if check!(is_alnum req.name) && check!(is_alnum req.pass) {
        if db.create(&req.name, &req.pass) {
            info!("User {} created", req.name);
            HttpResponse::Ok()
                .content_type(http::header::ContentType::plaintext())
                .body("user created")
        } else {
            warn!("Create user failed with given credentials");
            HttpResponse::BadRequest()
                .content_type(http::header::ContentType::plaintext())
                .body("given credentials resulted in conflicts")
        }
    } else {
        warn!("Given name or pass is not legal");
        HttpResponse::BadRequest()
            .content_type(http::header::ContentType::plaintext())
            .body("name or pass is not legal")
    }
}
