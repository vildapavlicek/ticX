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

use diesel::sql_types::Text;
// diesel::sql_function!(pgcrypto, crypt, (pass: Text, salt: Text) -> Text); // this is deprecated o.O
diesel::sql_function!(fn crypt(pass: Text, salt: Text) -> Text);

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    inner: PgPool,
}

impl Db {
    pub fn connect(uri: &str) -> DbResult<Db> {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        diesel::r2d2::Builder::new()
            .connection_timeout(std::time::Duration::from_secs(5)) // todo make this configurable
            .max_size(5) // todo make this configurable
            .build(manager)
            .map(|conn| Db { inner: conn })
            .map_err(|err| DbError::connection_error(uri, err))
    }

    fn get_conn(
        &self,
        action: &'static str,
    ) -> DbResult<diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.inner
            .get()
            .map_err(|err| DbError::connection_not_available(action, err))
    }

    pub fn select_users(&self) -> DbResult<Vec<User>> {
        users
            .load::<User>(&self.get_conn("select users")?)
            .map_err(|err| DbError::query_error("select users", err))
    }

    pub fn insert_user(&self, user: dbo::NewUser) -> DbResult<()> {
        /* diesel::insert_into(users_table)
        .values(&user)
        .execute(&self.get_conn("insert user")?)
        .map_err(|err| DbError::insert_error("users", err))
        .map(|rows_affected| {
            tracing::debug!(rows_affected, "inserted new user");
        }) */

        let query = diesel::insert_into(users_table).values((
            username.eq(user.username),
            password.eq(crypt(user.password, "gen_salt('bf', 8)")),
            firstname.eq(user.firstname),
            lastname.eq(user.lastname),
        ));

        tracing::trace!(?query, "constructed query");

        query
            .execute(&self.get_conn("insert user")?)
            .map_err(|err| DbError::insert_error("users", err))
            .map(|rows_affected| {
                tracing::debug!(rows_affected, "inserted new user");
            })
    }

    pub fn update_user(&self, user: &User) -> DbResult<()> {
        diesel::update(users_table.find(user.id))
            .set(user)
            .execute(&self.get_conn("update user")?)
            .map_err(|err| DbError::update_error("user", err))
            .map(|rows_affected| {
                tracing::debug!(%rows_affected, "updated user");
            })
    }

    pub fn select_tickets(&self) -> DbResult<Vec<Ticket>> {
        tickets
            .load::<Ticket>(&self.get_conn("select tickets")?)
            .map_err(|err| DbError::query_error("select tickets", err)) //todo we should probably limit this to some reasonable amount
    }

    pub fn insert_ticket(&self, ticket: dbo::NewTicket) -> DbResult<()> {
        diesel::insert_into(tickets_table)
            .values(&ticket)
            .execute(&self.get_conn("insert ticket")?)
            .map_err(|err| DbError::insert_error("tickets", err))
            .map(|rows_affected| {
                tracing::debug!(rows_affected, "inserted new ticket");
            })
    }

    pub fn update_ticket(&self, ticket: Ticket) -> DbResult<()> {
        diesel::update(tickets_table.find(ticket.id))
            .set(&ticket)
            .execute(&self.get_conn("update ticket")?)
            .map_err(|err| DbError::update_error("ticket", err))
            .map(|rows_affected| tracing::debug!(%rows_affected, "updated ticket"))
    }
}
