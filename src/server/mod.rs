use actix_web::web::scope;
use actix_web::{middleware::Logger, App};
use actix_web_opentelemetry::RequestTracing;

mod routes;

pub async fn start() {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(routes::user_routes())
            .service(routes::index)
            .wrap(RequestTracing::new())
    })
    .bind("127.0.0.1:8080")
    .expect("failed to bind to localhost:8080")
    .run()
    .await
    .expect("failed to start server");
}
