use crate::schema::{tickets, users};
use std::fmt::Formatter;

#[derive(Debug, Queryable, AsChangeset)]
pub struct Ticket {
    pub id: i32,
    pub author_id: i32,
    pub description: String,
    pub severity: i16,
    pub status: i16,
    pub created: chrono::NaiveDateTime,
}

impl Ticket {
    pub fn new(
        id: Option<i32>,
        author_id: i32,
        description: String,
        severity: i16,
        status: Option<i16>,
    ) -> Self {
        let now = chrono::Local::now();
        Ticket {
            id: id.unwrap_or(0),
            author_id,
            description,
            severity,
            status: status.unwrap_or(0),
            created: chrono::NaiveDateTime::from_timestamp(
                now.timestamp(),
                now.timestamp_subsec_nanos(),
            ),
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "tickets"]
pub struct NewTicket {
    pub(crate) author_id: i32,
    pub(crate) description: String,
    pub(crate) severity: i16,
    pub(crate) status: i16,
}

impl NewTicket {
    pub fn new(author_id: i32, description: String, severity: i16) -> Self {
        NewTicket {
            author_id,
            description,
            severity,
            status: 0,
        }
    }
}

#[derive(Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub created: chrono::NaiveDateTime,
}

impl User {
    pub fn new(
        id: Option<i32>,
        username: String,
        password: String,
        firstname: String,
        lastname: String,
    ) -> Self {
        let now = chrono::Local::now();
        User {
            id: id.unwrap_or(0),
            username,
            password,
            firstname,
            lastname,
            created: chrono::NaiveDateTime::from_timestamp(
                now.timestamp(),
                now.timestamp_subsec_micros(),
            ),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"*censored*")
            .field("firstname", &self.firstname)
            .field("lastname", &self.lastname)
            .field("created", &self.created)
            .finish()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
}

impl NewUser {
    pub fn new(username: String, password: String, firstname: String, lastname: String) -> Self {
        NewUser {
            username,
            password,
            firstname,
            lastname,
        }
    }
}

impl std::fmt::Debug for NewUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewUser")
            .field("username", &self.username)
            .field("password", &"*censored*")
            .field("firstname", &self.firstname)
            .field("lastname", &self.lastname)
            .finish()
    }
}
