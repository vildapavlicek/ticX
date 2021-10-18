use actix_web::{delete, get, post, put, web, Responder};
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

impl From<Ticket> for db::dbo::NewTicket {
    fn from(t: Ticket) -> Self {
        db::dbo::NewTicket::new(t.author_id, t.description, t.severity)
    }
}

#[get("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn get(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.select_ticket(id.into_inner())).await {
        Ok(ticket) => format!("got ticket: {:#?}", ticket),
        Err(e) => format!("failed to select ticket: {}", e),
    }
}

#[get("")]
#[tracing::instrument(skip(db))]
pub async fn get_all(db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.select_tickets()).await {
        Ok(tickets) => format!("got tickets: {:#?}", tickets),
        Err(e) => format!("failed to get all tickets: {}", e),
    }
}

#[post("")]
#[tracing::instrument(skip(db))]
pub async fn post(json: web::Json<Ticket>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.insert_ticket(json.into_inner().into())).await {
        Ok(_) => format!("inserted ticket"),
        Err(e) => format!("failed to insert ticket: {}", e),
    }
}

#[put("")]
#[tracing::instrument(skip(db))]
pub async fn put(json: web::Json<Ticket>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.update_ticket(json.into_inner().into())).await {
        Ok(_) => format!("updated ticket"),
        Err(e) => format!("failed to update ticket: {}", e),
    }
}

#[delete("/{id}")]
#[tracing::instrument(skip(db))]
pub async fn delete(id: web::Path<i32>, db: web::Data<Arc<Db>>) -> String {
    match web::block(move || db.delete_ticket(id.into_inner())).await {
        Ok(_) => format!("deleted ticket"),
        Err(e) => format!("failed to delete ticket: {}", e),
    }
}
