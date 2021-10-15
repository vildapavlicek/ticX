mod ticket;
mod token;
mod user;

use actix_web::get;

#[get("/")]
pub(super) async fn index() -> &'static str {
    "Hello world!"
}

macro_rules! routes {
    ($name:ident, $module:ident) => {
        pub(crate) fn $name() -> actix_web::Scope {
            actix_web::Scope::new(std::stringify!($module))
                .service($module::get)
                .service($module::post)
                .service($module::put)
                .service($module::delete)
        }
    };
    ($name:ident, $module:ident, $method_name:ident) => {
        pub(crate) fn $name() -> actix_web::Scope {
            actix_web::Scope::new(std::stringify!($module))
                .service($module::$method_name)
                .service($module::$method_name)
                .service($module::$method_name)
                .service($module::$method_name)
        }
    };
}

routes!(user_routes, user);
routes!(ticket_routes, ticket);

pub fn token_routes() -> actix_web::Scope {
    actix_web::Scope::new("token").service(token::get)
}

// pub(crate) fn user_routes() -> actix_web::Scope {
//     actix_web::Scope::new("user")
//         .service(user::get)
//         .service(user::post)
//         .service(user::put)
//         .service(user::delete)
// }
//
// pub(crate) fn ticket_routes() -> actix_web::Scope {
//     actix_web::Scope::new("ticket")
//         .service(ticket::get)
//         .service(ticket::post)
//         .service(ticket::put)
//         .service(ticket::delete)
// }
