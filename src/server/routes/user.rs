use actix_web::{delete, get, post, put, web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    id: Option<usize>,
    role: String,
}

#[get("/{id}")]
#[tracing::instrument]
pub async fn get(id: web::Path<usize>) -> String {
    format!("Got user id {}, name Antik", id)
}

#[post("")]
#[tracing::instrument]
pub async fn post(json: web::Json<User>) -> String {
    format!("creating new user: {:#?}", json)
}

#[put("")]
#[tracing::instrument]
pub async fn put(json: web::Json<User>) -> String {
    format!("updating existing user: {:#?}", json)
}

#[delete("/{id}")]
#[tracing::instrument]
pub async fn delete(id: web::Path<usize>) -> String {
    format!("deleting user id {}", id)
}
