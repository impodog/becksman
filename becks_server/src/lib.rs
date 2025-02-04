mod crew;
mod prelude;
mod user;

use prelude::*;

#[derive(Serialize, Deserialize)]
struct HelloNameRequest {
    name: String,
}

#[post("/test_name")]
async fn hello_name(info: web::Json<HelloNameRequest>) -> impl Responder {
    info!("Responding /test_name");
    format!("Hello, {}!", info.name)
}

#[get("/test")]
async fn hello() -> impl Responder {
    info!("Responding /test");
    HttpResponse::Ok().body("Hello world!")
}

pub async fn app() -> std::io::Result<()> {
    let db = Arc::new(becks_db::Db::connect());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(hello_name)
            .configure(user::config_user)
            .configure(crew::config_crew)
    })
    .bind(becks_db::CONFIG.server.addr)?
    .run()
    .await
}
