use crate::prelude::*;
use becks_convey::user::clean::*;
use std::sync::Arc;

impl Login {
    pub async fn update(&self) {
        self.client()
            .post(server_url!("user/update"))
            .json(&UpdateRequest {
                token: self.token(),
            })
            .send()
            .await
            .inspect_err(|err| {
                error!("When updating login for {:?}, {}", self.token(), err);
            })
            .ok();
    }
}

pub fn start_update_login(login: Arc<Login>) {
    tokio::spawn(async move {
        while !*login.end.lock().unwrap() {
            let future = login.update();
            tokio::time::sleep(CONFIG.request.update_relay).await;
            future.await;
        }
    });
}
