use crate::schema::{tickets, users};

#[derive(Debug, Queryable, AsChangeset)]
pub struct Ticket {
    pub(crate) id: i32,
    pub(crate) author_id: i32,
    pub(crate) description: String,
    pub(crate) severity: i16,
    pub(crate) status: i16,
    pub(crate) created: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "tickets"]
pub struct NewTicket<'a> {
    pub(crate) author_id: i32,
    pub(crate) description: &'a str,
    pub(crate) severity: i16,
    pub(crate) status: i16,
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

#[derive(Debug, Queryable, AsChangeset)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) created: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser /*<'a>*/ {
    /* pub(crate) username: &'a str,
    pub(crate) password: &'a str,
    pub(crate) firstname: &'a str,
    pub(crate) lastname: &'a str, */
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
}

impl NewUser /* <'a> */ {
    pub fn new(username: String, password: String, firstname: String, lastname: String) -> Self {
        NewUser {
            username,
            password,
            firstname,
            lastname,
        }
    }
}
