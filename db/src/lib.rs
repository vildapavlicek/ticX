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
diesel::sql_function!(fn crypt(pass: Text, salt: Text) -> Text);

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Db {
    inner: PgPool,
}

impl Db {
    #[tracing::instrument]
    pub fn connect(uri: &str) -> DbResult<Db> {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        tracing::trace!("ConnectionManager created");
        diesel::r2d2::Builder::new()
            .connection_timeout(std::time::Duration::from_secs(15)) // todo make this configurable
            .max_size(1) // todo make this configurable BUG if increased connection fails most of the time
            .build(manager)
            .map(|conn| {
                tracing::trace!("connected to DB");
                Db { inner: conn }
            })
            .map_err(|err| DbError::connection_error(uri, err))
    }

    #[tracing::instrument(skip(self))]
    fn get_conn(
        &self,
        action: &'static str,
    ) -> DbResult<diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.inner
            .get()
            .map_err(|err| DbError::connection_not_available(action, err))
    }

    #[tracing::instrument(skip(self))]
    pub fn select_users(&self) -> DbResult<Vec<User>> {
        users
            .load::<User>(&self.get_conn("select users")?)
            .map_err(|err| DbError::query_error("select users", err))
    }

    #[tracing::instrument(skip(self))]
    pub fn select_user(&self, user_id: i32) -> DbResult<dbo::User> {
        users_table
            .filter(schema::users::id.eq(user_id))
            .first::<User>(&self.get_conn("select user")?)
            .map_err(|err| DbError::query_error("select user", err))
    }

    #[tracing::instrument(skip(self))]
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

        // tracing::trace!(?query, "insert query"); //also logs password, so uncomment if you need to debug query

        query
            .execute(&self.get_conn("insert user")?)
            .map_err(|err| DbError::insert_error("users", err))
            .map(|rows_affected| {
                tracing::debug!(rows_affected, "inserted new user");
            })
    }

    #[tracing::instrument(skip(self))]
    pub fn update_user(&self, user: &User) -> DbResult<()> {
        diesel::update(users_table.find(user.id))
            .set(user)
            .execute(&self.get_conn("update user")?)
            .and_then(|rows_affected| {
                tracing::debug!(%rows_affected, "updated user");
                match rows_affected {
                    0 => Err(diesel::NotFound),
                    _ => Ok(()),
                }
            })
            .map_err(|err| DbError::update_error("user", err))
    }

    #[tracing::instrument(skip(self))]
    pub fn delete_user(&self, user_id: i32) -> DbResult<usize> {
        diesel::delete(users_table.filter(crate::schema::users::id.eq(user_id)))
            .execute(&self.get_conn("delete user")?)
            .and_then(|rows_affected| {
                tracing::debug!(%rows_affected, "delete user");
                match rows_affected {
                    0 => Err(diesel::NotFound),
                    _ => Ok(rows_affected),
                }
            })
            .map_err(|err| DbError::query_error("delete user", err))
    }

    #[tracing::instrument(skip(self))]
    pub fn select_tickets(&self) -> DbResult<Vec<Ticket>> {
        tickets
            .load::<Ticket>(&self.get_conn("select tickets")?)
            .map_err(|err| DbError::query_error("select tickets", err)) //todo we should probably limit this to some reasonable amount
    }

    #[tracing::instrument(skip(self))]
    pub fn select_ticket(&self, ticket_id: i32) -> DbResult<Ticket> {
        tickets_table
            .filter(schema::tickets::id.eq(ticket_id))
            .first::<Ticket>(&self.get_conn("select ticket")?)
            .map_err(|err| DbError::query_error("select ticket", err))
    }

    #[tracing::instrument(skip(self))]
    pub fn insert_ticket(&self, ticket: dbo::NewTicket) -> DbResult<()> {
        diesel::insert_into(tickets_table)
            .values(&ticket)
            .execute(&self.get_conn("insert ticket")?)
            .map_err(|err| DbError::insert_error("tickets", err))
            .map(|rows_affected| {
                tracing::debug!(rows_affected, "inserted new ticket");
            })
    }

    #[tracing::instrument(skip(self))]
    pub fn update_ticket(&self, ticket: Ticket) -> DbResult<()> {
        diesel::update(tickets_table.find(ticket.id))
            .set(&ticket)
            .execute(&self.get_conn("update ticket")?)
            .map_err(|err| DbError::update_error("ticket", err))
            .map(|rows_affected| tracing::debug!(%rows_affected, "updated ticket"))
    }

    #[tracing::instrument(skip(self))]
    pub fn delete_ticket(&self, ticket_id: i32) -> DbResult<usize> {
        diesel::delete(tickets_table.find(ticket_id))
            .execute(&self.get_conn("delete ticket")?)
            .map_err(|err| DbError::query_error("delete ticket", err))
    }

    #[tracing::instrument(skip(self, pwd))]
    pub fn check_credentials(&self, usr: &str, pwd: &str) -> DbResult<dbo::User> {
        let query = users_table
            .filter(schema::users::username.eq(usr))
            .filter(schema::users::password.eq(crypt(pwd, "gen_salt('bf', 8)")));

        // tracing::trace!(?query, "user authentication query"); this will also log passwords, so we cannot really keep it here

        let mut found = query
            .load::<dbo::User>(&self.get_conn("user auth")?)
            .map_err(|err| DbError::query_error("delete ticket", err))?;

        match found.len() {
            0 => Err(DbError::not_found("user")),
            1 => Ok(found.pop().unwrap()),
            // if we find more users with same name and password then we've reached some kind of invalid state
            _ => Err(DbError::InvalidResult),
        }
    }
}
