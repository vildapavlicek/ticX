mod app;
mod server;
mod tracer;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = tracer::init();
    tracing::info!(version = env!("CARGO_PKG_VERSION"), "ticX App started");

    crate::app::TicXApp::run().await?;

    Ok(())
}
