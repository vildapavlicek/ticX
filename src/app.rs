use crate::server;
use std::sync::Arc;

pub struct TicXApp {}

impl TicXApp {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let db = Arc::new(db::Db::connect(
            "postresql://user::password@localhost:5432/ticx",
        )?);
        server::start(db).await
    }
}
