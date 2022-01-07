use crate::errors::{TicxError, TicxResult};
use actix_web::{middleware::Logger, App};
use actix_web_opentelemetry::RequestTracing;
use std::str::FromStr;
use std::sync::Arc;

mod middlewares;
mod routes;

#[tracing::instrument(skip(db))]
pub async fn start(db: Arc<db::Db>) -> Result<(), Box<dyn std::error::Error>> {
    let secret = Arc::new(routes::auth::Secret(String::from("my_super_jwt_secret")));
    tracing::trace!("starting server");
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(db.clone())
            .data(secret.clone())
            .service(
                actix_web::Scope::new("/api")
                    .service(routes::index)
                    .service(routes::user_routes())
                    .service(routes::ticket_routes())
                    .wrap(middlewares::JWTValidationMiddleware {
                        secret: secret.clone(),
                    }), // .service(routes::token_routes()), // .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer()),
            )
            .service(
                routes::auth_routes().wrap(middlewares::BasicAuthMiddleware { db: db.clone() }),
            )
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
    })
    .bind("127.0.0.1:8080")
    .expect("failed to bind to localhost:8080")
    .run()
    .await?;
    Ok(())
}
