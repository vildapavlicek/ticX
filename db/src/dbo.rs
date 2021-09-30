#[derive(Debug, sqlx::FromRow)]
pub struct Ticket {
    id: i32,
    author_id: i32,
    description: String,
    severity: i16,
    status: i16,
    created: chrono::Local,
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    firstname: String,
    lastname: String,
    created: chrono::Local,
}
