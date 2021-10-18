use actix_web::{delete, get, post, put, web, Responder};
use db::Db;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::Instrument;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    password: String,
    firstname: String,
    lastname: String,
    id: Option<i32>,
    role: String,
}

impl From<User> for db::dbo::NewUser {
    fn from(user: User) -> db::dbo::NewUser {
        db::dbo::NewUser::new(user.username, user.password, user.firstname, user.lastname)
    }
}

impl From<User> for db::dbo::User {
    fn from(user: User) -> Self {
        db::dbo::User::new(
            user.id,
            user.username,
            user.password,
            user.firstname,
            user.lastname,
        )
    }
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.select_user(id.into_inner())).await {
        Ok(user) => format!("found user {:?}", user),
        Err(e) => format!("error selecting user {}", e),
    }
}

#[get("")]
#[tracing::instrument(skip(db))]
pub async fn get_all(db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.select_users()).await {
        Ok(users) => format!("selected users: {:#?}", users),
        Err(e) => format!("failed to select users: {}", e),
    }
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<User>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.insert_user(json.into_inner().into())).await {
        Ok(_) => String::from("insert OK"),
        Err(e) => format!("insert failed: {}", e),
    }
}

#[put("")]
#[tracing::instrument(skip(db))]
pub async fn put(json: web::Json<User>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.update_user(&json.into_inner().into())).await {
        Ok(_) => format!("updated user"),
        Err(e) => format!("failed to update user {}", e),
    }
}

#[delete("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn delete(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.delete_user(id.into_inner())).await {
        Ok(rows_affected) => format!("deleted {}", rows_affected),
        Err(e) => format!("failed to delete user {}", e),
    }
}
