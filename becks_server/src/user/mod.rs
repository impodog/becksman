mod acq;
mod auth;

use crate::prelude::*;

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
            .service(auth::create_user),
    );
}
