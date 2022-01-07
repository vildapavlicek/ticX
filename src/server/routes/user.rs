use crate::errors::{TicxError, TicxResult};
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse};
use db::Db;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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

impl From<db::dbo::User> for User {
    fn from(db_user: db::dbo::User) -> Self {
        User {
            username: db_user.username,
            password: "*censored*".into(),
            firstname: db_user.firstname,
            lastname: db_user.lastname,
            id: Some(db_user.id),
            role: "NotImplemented".into(),
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username)
            .field("password", &"*censored*")
            .field("firstname", &self.firstname)
            .field("lastname", &self.lastname)
            .field("id", &self.id)
            .field("role", &self.role)
            .finish()
    }
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> TicxResult<Json<User>> {
    tracing::trace!("requested user information");
    web::block(move || db.select_user(id.into_inner()))
        .await
        .map(|u| Json(u.into()))
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[get("")]
#[tracing::instrument(skip(db))]
pub async fn get_all(db: web::Data<Arc<Db>>) -> TicxResult<Json<Vec<User>>> {
    tracing::trace!("requested all users information");
    web::block(move || db.select_users())
        .await
        .map(|v| Json(v.into_iter().map(User::from).collect::<Vec<User>>()))
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<User>, db: web::Data<Arc<Db>>) -> TicxResult<HttpResponse> {
    tracing::trace!("requested to create new user");
    web::block(move || db.insert_user(json.into_inner().into()))
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[put("")]
#[tracing::instrument(skip(db))]
pub async fn put(json: web::Json<User>, db: web::Data<Arc<Db>>) -> TicxResult<HttpResponse> {
    tracing::trace!("requested to update user");
    web::block(move || db.update_user(&json.into_inner().into()))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[delete("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn delete(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> TicxResult<HttpResponse> {
    tracing::trace!("requested to delete user");
    web::block(move || db.delete_user(id.into_inner()))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|err| TicxError::DbFail(err.to_string()))
}
