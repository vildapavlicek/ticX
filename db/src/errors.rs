pub type DbResult<T> = Result<T, DbError>;

#[derive(thiserror::Error)]
pub enum DbError {}
