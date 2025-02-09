use crate::prelude::*;
use becks_convey::user::auth::*;
use becks_convey::user::clean::UpdateRequest;
use becks_db::{Db, CONFIG};
use std::sync::Arc;

async fn clean_up_users(db: &Db) {
    let mut futures = Vec::new();
    db.login_map().iter().for_each(|(token, login)| {
        if login.duration_since_last_update() > CONFIG.user.timeout {
            info!(
                "Disconnecting user with token {:?} after {:?}",
                token,
                login.duration_since_last_update()
            );
            futures.push(super::auth::log_out_work(
                LogoutRequest { token: *token },
                db,
            ));
        }
    });
    for future in futures.into_iter() {
        future.await;
    }
}

pub fn start_clean_up(db: Arc<Db>) {
    tokio::spawn(async move {
        loop {
            let future = clean_up_users(db.as_ref());
            tokio::time::sleep(CONFIG.user.timeout).await;
            future.await;
        }
    });
}

#[post("/update")]
pub(super) async fn update_user(req: web::Json<UpdateRequest>, db: DbData) -> HttpResponse {
    debug!("Attempt to update user with token {:?}", req.token);
    let login = extract_login!(db, &req.token);
    login.update_time();
    HttpResponse::Ok()
        .content_type(http::header::ContentType::plaintext())
        .body("user time updated")
}
