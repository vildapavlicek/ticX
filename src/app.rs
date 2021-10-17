use crate::server;
use std::sync::Arc;

pub struct TicXApp {}

impl TicXApp {
    #[tracing::instrument]
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        tracing::trace!("initializing DB");

        let db_uri = dotenv::var("POSTGRES_URI")
            .unwrap_or_else(|_| String::from("postgres://user:password@localhost:5432/ticx"));
        // let result = actix_web::web::block(move || db::Db::connect(db_uri.as_str())).await?;
        // tracing::trace!("after DB");
        // let db = Arc::new(result);
        // let db = Arc::new(db::Db::connect(db_uri.as_str())?);
        let db = Arc::new(actix_web::web::block(move || db::Db::connect(db_uri.as_str())).await?);
        tracing::trace!("after DB");
        server::start(db).await
        // server::start().await
        // Ok(())
    }
}
