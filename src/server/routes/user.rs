use actix_web::{delete, get, post, put, web, Responder};
use db::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    id: Option<usize>,
    role: String,
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<usize>, db: web::Data<Db>) -> String {
    format!("Got user id {}, name Antik", id)
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<User>, db: web::Data<Db>) -> String {
    format!("creating new user: {:#?}", json)
}

#[put("")]
#[tracing::instrument(skip(db))]
pub async fn put(json: web::Json<User>, db: web::Data<Db>) -> String {
    format!("updating existing user: {:#?}", json)
}

#[delete("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn delete(id: web::Path<usize>, db: web::Data<Db>) -> String {
    format!("deleting user id {}", id)
}
