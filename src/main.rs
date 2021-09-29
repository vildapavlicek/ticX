mod app;
mod server;
mod tracer;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let _guard = tracer::init();

    tracing::info!(version = env!("CARGO_PKG_VERSION"), "ticX App started");

    server::start().await;

    Ok(())
}
