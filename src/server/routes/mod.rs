pub(super) mod auth;
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
                .service($module::get_all)
                .service($module::post)
                .service($module::put)
                .service($module::delete)
        }
    };
    ($name:ident, $module:ident, $($method_name:ident)&+) => {
        pub(crate) fn $name() -> actix_web::Scope {
            let mut scope = actix_web::Scope::new(std::stringify!($module));
            $(
                scope = scope.service($module::$method_name);
            )*
            scope
        }
    };
}

routes!(user_routes, user);
routes!(ticket_routes, ticket);
routes!(auth_routes, auth, login);
// routes!(token_routes, token, get);

// routes!(
//     test_name,
//     test_module,
//     get_method & post_method & put_method
// );
