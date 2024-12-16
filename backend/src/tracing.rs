use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::Resource;
use opentelemetry::trace::TraceError;
use opentelemetry::KeyValue;
use opentelemetry_aws::trace::XrayPropagator;
use opentelemetry_jaeger::PipelineBuilder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    let tracer = PipelineBuilder::default()
        .with_service_name("bidding_app")
        .with_trace_config(sdktrace::config().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "bidding_app"),
        ])))
        .install_batch(opentelemetry::runtime::Tokio)?;
    opentelemetry::global::set_text_map_propagator(XrayPropagator::default());
    Ok(tracer)
}

pub fn init_tracing() {
    let tracer = init_tracer().expect("Failed to initialize tracer");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
