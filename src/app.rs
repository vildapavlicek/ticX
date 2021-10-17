use crate::server;
use std::sync::Arc;

pub struct TicXApp {}

impl TicXApp {
    #[tracing::instrument]
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        tracing::trace!("initializing DB");
        let db = Arc::new(db::Db::connect(
            "postres://user::password@localhost:5432/ticx",
        )?);
        server::start(db).await
    }
}
