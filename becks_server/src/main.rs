#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    becks_server::app().await?;

    becks_db::save_config();

    Ok(())
}
