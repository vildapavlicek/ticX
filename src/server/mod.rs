use actix_web::{middleware::Logger, App};
use actix_web_opentelemetry::RequestTracing;
use std::sync::Arc;

mod routes;

#[tracing::instrument(skip(db))]
pub async fn start(db: Arc<db::Db>) -> Result<(), Box<dyn std::error::Error>> {
    tracing::trace!("starting server");
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(db.clone())
            .service(routes::index)
            .service(routes::user_routes())
            .service(routes::ticket_routes())
            .service(routes::token_routes())
            .wrap(RequestTracing::new())
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")
    .expect("failed to bind to localhost:8080")
    .run()
    .await?;
    Ok(())
}

// #[tracing::instrument]
// pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
//     tracing::trace!("starting server");
//     actix_web::HttpServer::new(move || {
//         actix_web::App::new()
//             .service(routes::index)
//             .service(routes::user_routes())
//             .service(routes::ticket_routes())
//             .service(routes::token_routes())
//             .wrap(RequestTracing::new())
//             .wrap(Logger::default())
//     })
//     .bind("127.0.0.1:8080")
//     .expect("failed to bind to localhost:8080")
//     .run()
//     .await?;
//     Ok(())
// }
