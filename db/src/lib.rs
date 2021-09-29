pub mod errors;

pub async fn connect(uri: &str) {
    sqlx::pool::Pool::connect(uri)
}
