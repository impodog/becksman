mod acq;
mod auth;
mod clean;

use crate::prelude::*;
pub use clean::start_clean_up;

#[get("/test")]
async fn test() -> impl Responder {
    info!("Responding /user/test");
    HttpResponse::Ok().body("User module is running!")
}

pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(test)
            .service(auth::log_in)
            .service(auth::log_out)
            .service(auth::create_user)
            .service(clean::update_user),
    );
}
