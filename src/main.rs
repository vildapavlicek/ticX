mod app;
mod errors;
mod metrics;
mod server;
mod tracer;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _guard = tracer::init();
    tracing::info!(version = env!("CARGO_PKG_VERSION"), "ticX App starting");

    crate::app::TicXApp::run().await?;
    tracing::trace!("server shutdown");

    Ok(())
}
