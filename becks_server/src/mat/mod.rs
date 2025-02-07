mod acquire;
mod create;
mod modify;
mod query;

use crate::prelude::*;

#[get("/test")]
async fn test() -> impl Responder {
    debug!("Responding /mat/test");
    HttpResponse::Ok().body("Match module is running!")
}

pub fn config_mat(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mat")
            .service(test)
            .service(create::create_mat)
            .service(acquire::acquire_mat)
            .service(modify::modify_mat)
            .service(query::query_mat),
    );
}
