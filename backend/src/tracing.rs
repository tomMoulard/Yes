use opentelemetry::sdk::trace as sdktrace;
use opentelemetry::sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_aws::trace::XrayPropagator;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn init_tracer() -> sdktrace::Tracer {
    let exporter = opentelemetry_aws::new_pipeline()
        .with_service_name("bidding_app")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Error initializing AWS X-Ray exporter");

    let tracer = sdktrace::Tracer::builder()
        .with_exporter(exporter)
        .with_resource(Resource::new(vec![KeyValue::new("service.name", "bidding_app")]))
        .build();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    tracing::subscriber::set_global_default(subscriber).expect("Error setting global subscriber");

    opentelemetry::global::set_text_map_propagator(XrayPropagator::default());

    tracer
}
