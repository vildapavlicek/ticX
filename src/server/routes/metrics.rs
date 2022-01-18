use crate::errors::{TicxError, TicxResult};
use actix_web::get;
use prometheus::Encoder;

#[get("")]
pub async fn prometheus_metrics() -> TicxResult<String> {
    crate::metrics::VALID_REQUESTS_COUNTER.inc();

    let mut buffer = vec![];
    let encoder = prometheus::TextEncoder::new();
    let metrics_family = prometheus::gather();

    encoder
        .encode(&metrics_family, &mut buffer)
        .map_err(|err| TicxError::GenericError {
            error: err.to_string(),
            what: "getting metrics",
        })?;

    let data = String::from_utf8(buffer).map_err(|err| TicxError::GenericError {
        error: err.to_string(),
        what: "metrics to String",
    })?;

    Ok(data)
}
