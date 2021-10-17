use actix_web::{middleware::Logger, App};
use actix_web_opentelemetry::RequestTracing;
use std::sync::Arc;

mod routes;

pub async fn start(db: Arc<db::Db>) -> Result<(), Box<dyn std::error::Error>> {
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(routes::index)
            .service(routes::user_routes())
            .service(routes::ticket_routes())
            .service(routes::token_routes())
            .data(db.clone())
            .wrap(RequestTracing::new())
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")
    .expect("failed to bind to localhost:8080")
    .run()
    .await?;
    Ok(())
}
