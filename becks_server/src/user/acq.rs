#[macro_export]
macro_rules! extract_login {
    ($db: expr, $token: expr) => {
        if let Some(login) = $db.get_login($token) {
            login
        } else {
            error!("Unable to find record for log-in token {:?}", $token);
            return HttpResponse::Unauthorized()
                .content_type(http::header::ContentType::plaintext())
                .body("unable to find a record for given token");
        }
    };
}
