pub mod errors;
use errors::{DbError, DbResult};

pub struct Db {
    inner: sqlx::Pool<sqlx::Postgres>,
}

impl Db {
    pub async fn connect(uri: &str) -> DbResult<sqlx::Pool<sqlx::Postgres>> {
        sqlx::postgres::PgPool::connect(uri)
        // todo add migration here to run
    }
}
