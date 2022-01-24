use prometheus::{
    labels, opts, register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec,
};

pub const DB_TABLE_USERS: &str = "USERS";
pub const DB_TABLE_TICKETS: &str = "TICKETS";

lazy_static::lazy_static! {
    pub static ref HTTP_REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!("http_request_total", "counts number of received requests", &["method"]).unwrap();
    pub static ref HTTP_RESPONSE_COUNTER: IntCounterVec = register_int_counter_vec!("http_response_total", "counts number of responses sent", &["status_code"]).unwrap();
    pub static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!("http_request_duration_seconds", "measurement of how long it took to process request", &["handler"]).unwrap();
    pub static ref DB_QUERY_HISTOGRAM: HistogramVec = register_histogram_vec!("db_query_duration_seconds", "measurement how long it takes to process DB query", &["table", "query"]).unwrap();
}
