#[macro_use]
extern crate diesel;

pub mod dbo;
pub mod errors;

use diesel::{pg::PgConnection, prelude::*};
use errors::{DbError, DbResult};

pub struct Db {
    // inner: sqlx::Pool<sqlx::Postgres>,
    inner: PgConnection,
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
        PgConnection::establish(uri)
            .map_err(|err| DbError::connection_error(uri, err))
            .and_then(|conn| Ok(Db { inner: conn }))
    }
}
