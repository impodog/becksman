mod create;
mod modify;

use crate::prelude::*;

#[get("/test")]
async fn test() -> impl Responder {
    debug!("Responding /mat/test");
    HttpResponse::Ok().body("Match module is running!")
}

pub fn config_mat(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/mat").service(test).service(create::create_mat));
}
