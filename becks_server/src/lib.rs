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
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(becks_db::Db::connect()))
            .service(hello)
            .service(hello_name)
            .configure(user::config_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
