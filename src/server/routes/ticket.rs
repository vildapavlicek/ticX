use actix_web::{delete, get, post, put, web, Responder};
use db::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticket {
    author_id: usize,
    description: String,
    severity: u8,
    status: u8,
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<usize>, db: web::Data<Db>) -> String {
    format!("Got user id {}, name Antik", id)
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<Ticket>, db: web::Data<Db>) -> String {
    format!("creating new user: {:#?}", json)
}

#[put("")]
#[tracing::instrument(skip(db))]
pub async fn put(json: web::Json<Ticket>, db: web::Data<Db>) -> String {
    format!("updating existing user: {:#?}", json)
}

#[delete("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn delete(id: web::Path<usize>, db: web::Data<Db>) -> String {
    format!("deleting user id {}", id)
}
