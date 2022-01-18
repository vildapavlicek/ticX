use prometheus::{register_counter, register_int_counter, Counter, IntCounter};

lazy_static::lazy_static! {
    pub static ref RESPONSE_COUNTER: IntCounter = register_int_counter!("RESPONSE_COUNTER", "Count of responses").unwrap();

    pub static ref OK_COUNTER: IntCounter = register_int_counter!("OK_COUNTER", "Count of 200 OK responses").unwrap();
    pub static ref BAD_REQUEST_COUNTER: IntCounter = register_int_counter!("BAD_REQUEST", "Count of 400 responses").unwrap();
    pub static ref UNAUTHORIZED_COUNTER: IntCounter = register_int_counter!("UNAUTHORIZED", "Count of 401 responses").unwrap();
    pub static ref NOT_FOUND_COUNTER: IntCounter = register_int_counter!("NOT_FOUND", "Count of 404 responses").unwrap();
    pub static ref INTERNAL_SRV_ERR_COUNTER: IntCounter = register_int_counter!("INTERNAL_SERVER_ERROR", "Count of 500 responses").unwrap();

    pub static ref ERROR_RESPONSE_COUNTER: IntCounter = register_int_counter!("ERROR_RESPONSES", "Count of ERROR responses").unwrap();

    pub static ref REQUESTS_COUNTER: IntCounter = register_int_counter!("REQUESTS", "Count of requests accepted by server").unwrap();

    pub static ref VALID_REQUESTS_COUNTER: IntCounter = register_int_counter!("VALID_REQUESTS", "Count of requests that passed all middleware checks").unwrap();
    pub static ref REQUEST_PROCESS_TIME_TOTAL: Counter = register_counter!("REQUEST_PROCESS_TIME_TOTAL", "Sum of how long it took for server to process requests").unwrap();
}
