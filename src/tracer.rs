use opentelemetry::global;
use opentelemetry::sdk::{export::trace::stdout, propagation::TraceContextPropagator};
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init() -> tracing_appender::non_blocking::WorkerGuard {
    // global::set_text_map_propagator(TraceContextPropagator::new());

    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    let tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("ticX_server")
        .install_simple()
        .expect("failed to initialize zipkin pipeline");

    let file_appender = tracing_appender::rolling::daily("", "ticX.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    Registry::default()
        .with(tracing_subscriber::EnvFilter::new(
            "actix_web=trace,ticx=trace,db=trace, diesel=trace",
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false),
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    _guard
}
