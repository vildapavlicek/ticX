use crate::errors::{TicxError, TicxResult};
use actix_web::web::Json;
use actix_web::{delete, get, post, put, web, HttpResponse};
use db::Db;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticket {
    id: Option<i32>,
    author_id: i32,
    description: String,
    severity: i16,
    status: Option<i16>,
}

impl From<Ticket> for db::dbo::Ticket {
    fn from(t: Ticket) -> Self {
        db::dbo::Ticket::new(t.id, t.author_id, t.description, t.severity, t.status)
    }
}

impl From<db::dbo::Ticket> for Ticket {
    fn from(t: db::dbo::Ticket) -> Self {
        Ticket {
            id: Some(t.id),
            author_id: t.author_id,
            description: t.description,
            severity: t.severity,
            status: Some(t.status),
        }
    }
}

impl From<Ticket> for db::dbo::NewTicket {
    fn from(t: Ticket) -> Self {
        db::dbo::NewTicket::new(t.author_id, t.description, t.severity)
    }
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> TicxResult<Json<Ticket>> {
    web::block(move || db.select_ticket(id.into_inner()))
        .await
        .map(|t| Json(t.into()))
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[get("")]
#[tracing::instrument(skip(db))]
pub async fn get_all(db: web::Data<Arc<Db>>) -> TicxResult<Json<Vec<Ticket>>> {
    tracing::trace!("requested all tickets");
    web::block(move || db.select_tickets())
        .await
        .map(|t| Json(t.into_iter().map(Ticket::from).collect::<Vec<Ticket>>()))
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<Ticket>, db: web::Data<Arc<Db>>) -> TicxResult<HttpResponse> {
    web::block(move || db.insert_ticket(json.into_inner().into()))
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[put("")]
#[tracing::instrument(skip(db))]
pub async fn put(json: web::Json<Ticket>, db: web::Data<Arc<Db>>) -> TicxResult<HttpResponse> {
    web::block(move || db.update_ticket(json.into_inner().into()))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|err| TicxError::DbFail(err.to_string()))
}

#[delete("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn delete(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> TicxResult<HttpResponse> {
    web::block(move || db.delete_ticket(id.into_inner()))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|err| TicxError::DbFail(err.to_string()))
}
