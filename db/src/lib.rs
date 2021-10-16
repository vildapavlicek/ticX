#[macro_use]
extern crate diesel;

pub mod dbo;
pub mod errors;
mod schema;

use crate::schema::{
    tickets::{dsl::*, table as tickets_table},
    users::{dsl::*, table as users_table},
};
use dbo::{Ticket, User};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use errors::{DbError, DbResult};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    // inner: sqlx::Pool<sqlx::Postgres>,
    inner: PgPool,
}

impl Db {
    // pub async fn connect(uri: &str) -> DbResult<sqlx::Pool<sqlx::Postgres>> {
    //     let pool = sqlx::postgres::PgPool::connect(uri)
    //         .await
    //         .map_err(|err| DbError::connection_error(uri, err))?;
    //
    //     sqlx::migrate!("./migrations").run(&pool).await.map_err(|err|)
    //     // todo add migration here to run
    // }

    pub fn connect(uri: &str) -> DbResult<Db> {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        diesel::r2d2::Builder::new()
            .connection_timeout(std::time::Duration::from_secs(5)) // todo make this configurable
            .max_size(5) // todo make this configurable
            .build(manager)
            .and_then(|conn| Ok(Db { inner: conn }))
            .map_err(|err| DbError::connection_error(uri, err))
    }

    pub fn get_users(&self) -> DbResult<Vec<User>> {
        users
            .load::<User>(
                &self
                    .inner
                    .get()
                    .map_err(|err| DbError::connection_not_available("select users", err))?,
            )
            .map_err(|err| DbError::query_error("select users", err))
    }

    pub fn get_tickets(&self) -> DbResult<Vec<Ticket>> {
        tickets
            .load::<Ticket>(
                &self
                    .inner
                    .get()
                    .map_err(|err| DbError::connection_not_available("select tickets", err))?,
            )
            .map_err(|err| DbError::query_error("select tickets", err)) //todo we should probably limit this to some reasonable amount
    }

    pub fn insert_user(&self, user: dbo::NewUser) -> DbResult<()> {
        diesel::insert_into(users_table)
            .values(&user)
            .execute(
                &self
                    .inner
                    .get()
                    .map_err(|err| DbError::connection_not_available("insert user", err))?,
            )
            .map_err(|err| DbError::insert_error("users", err))
            .and_then(|rows_affected| { tracing::debug!(rows_affected, "inserted new user"); Ok(())})

    }

    pub fn insert_ticket(&self, ticket: dbo::NewTicket) -> DbResult<()> {
        diesel::insert_into(tickets_table)
            .values(&ticket)
            .execute(
                &self
                    .inner
                    .get()
                    .map_err(|err| DbError::connection_not_available("insert ticket", err))?,
            )
            .map_err(|err| DbError::insert_error("tickets", err))
            .and_then(|rows_affected| { tracing::debug!(rows_affected, "inserted new ticket"); Ok(())})

    }
}
