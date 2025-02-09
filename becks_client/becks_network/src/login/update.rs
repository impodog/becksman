use crate::prelude::*;
use becks_convey::user::clean::*;
use std::sync::Arc;

async fn update_login(login: &Login) {
    login
        .client()
        .post(server_url!("user/update"))
        .json(&UpdateRequest {
            token: login.token(),
        })
        .send()
        .await
        .inspect_err(|err| {
            error!("When updating login for {:?}, {}", login.token(), err);
        })
        .ok();
}

pub fn start_update_login(login: Arc<Login>) {
    tokio::spawn(async move {
        while !*login.end.lock().unwrap() {
            let future = update_login(login.as_ref());
            tokio::time::sleep(CONFIG.request.update_relay).await;
            future.await;
        }
    });
}
