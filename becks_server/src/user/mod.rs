mod auth;

use crate::prelude::*;

#[get("/test")]
async fn test() -> impl Responder {
    info!("Responding /user/test");
    HttpResponse::Ok().body("User module is running!")
}

// this function could be located in a different module
pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(test).service(auth::log_in));
}
