mod crew;
mod mat;
mod poster;
mod prelude;
mod user;

use prelude::*;

#[derive(Serialize, Deserialize)]
struct HelloNameRequest {
    name: String,
}

#[post("/test_name")]
async fn hello_name(info: web::Json<HelloNameRequest>) -> impl Responder {
    trace!("Responding /test_name");
    format!("Hello, {}!", info.name)
}

#[get("/test")]
async fn hello() -> impl Responder {
    trace!("Responding /test");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn app() -> std::io::Result<()> {
    let db = Arc::new(becks_db::Db::connect());
    user::start_clean_up(db.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(hello_name)
            .configure(user::config_user)
            .configure(crew::config_crew)
            .configure(mat::config_mat)
            .configure(poster::config_poster)
    })
    .bind(becks_db::CONFIG.server.addr)?
    .run()
    .await
}
