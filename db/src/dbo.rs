use crate::schema::{tickets, users};

#[derive(Debug, Queryable)]
pub struct Ticket {
    id: i32,
    author_id: i32,
    description: String,
    severity: i16,
    status: i16,
    created: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    author_id: i32,
    description: &'a str,
    severity: i16,
    status: i16,
}

impl<'a> NewTicket<'a> {
    pub fn new(author_id: i32, description: &'a str, severity: i16) -> Self {
        NewTicket {
            author_id,
            description,
            severity,
            status: 0,
        }
    }
}

#[derive(Debug, Queryable)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    firstname: String,
    lastname: String,
    created: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    username: &'a str,
    password: &'a str,
    firstname: &'a str,
    lastname: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(
        username: &'a str,
        password: &'a str,
        firstname: &'a str,
        lastname: &'a str,
    ) -> Self {
        NewUser {
            username,
            password,
            firstname,
            lastname,
        }
    }
}
