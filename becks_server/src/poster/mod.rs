mod acquire;
mod create;
mod modify;
mod query;

use crate::prelude::*;

#[get("/test")]
async fn test() -> impl Responder {
    debug!("Responding /poster/test");
    HttpResponse::Ok().body("Poster module is running!")
}

pub fn config_poster(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/poster")
            .service(test)
            .service(create::create_poster)
            .service(acquire::acquire_poster)
            .service(modify::modify_poster)
            .service(query::query_poster),
    );
}
