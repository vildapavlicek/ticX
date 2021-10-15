use actix_web::{delete, get, post, put, web, HttpRequest, Responder};

#[get("")]
pub(super) async fn get(auth: actix_web_httpauth::extractors::basic::BasicAuth) -> String {
    // todo: verify user if exists in DB and if so, return token
    println!("{:?}, {:?}", auth.user_id(), auth.password());
    "ok".into()
}
