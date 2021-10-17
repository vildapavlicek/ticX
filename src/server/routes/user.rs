use actix_web::{delete, get, post, put, web, Responder};
use db::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    password: String,
    firstname: String,
    lastname: String,
    id: Option<usize>,
    role: String,
}

impl From<User> for db::dbo::NewUser {
    fn from(user: User) -> db::dbo::NewUser {
        db::dbo::NewUser::new(user.username, user.password, user.firstname, user.lastname)
    }
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<usize>, db: web::Data<Db>) -> String {
    format!("Got user id {}, name Antik", id)
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<User>, db: web::Data<Db>) -> String {
    match web::block(move || db.insert_user(json.into_inner().into())).await {
        Ok(_) => String::from("insert OK"),
        Err(e) => format!("insert failed: {}", e),
    }
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
