use crate::server;
use std::sync::Arc;

pub struct TicXApp {}

impl TicXApp {
    #[tracing::instrument]
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        tracing::trace!("initializing DB");

        let db_uri = dotenv::var("POSTGRES_URI")
            .unwrap_or_else(|_| String::from("postres://user::password@localhost:5432/ticx"));
        let db = Arc::new(db::Db::connect(db_uri.as_str())?);
        server::start(db).await
    }
}
