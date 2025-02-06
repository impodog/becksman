mod create;
mod delete;
mod modify;
mod query;

use crate::prelude::*;

#[get("/test")]
async fn test() -> impl Responder {
    debug!("Responding /crew/test");
    HttpResponse::Ok().body("Crew module is running!")
}

pub fn config_crew(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/crew")
            .service(test)
            .service(create::create_crew)
            .service(modify::modify_crew)
            .service(modify::acquire_crew)
            .service(query::query_by_crew)
            .service(delete::delete_crew),
    );
}
