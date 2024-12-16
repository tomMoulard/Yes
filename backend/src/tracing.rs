use opentelemetry::global;
use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime::Tokio, trace as sdktrace};
use opentelemetry_stdout::SpanExporter;

use opentelemetry::{
    trace::{SpanKind, TraceContextExt, Tracer},
    Context, KeyValue,
};
use crate::config::ServiceConfig;

// pub fn init_tracer() -> sdktrace::TracerProvider {
    // global::set_text_map_propagator(TraceContextPropagator::new());
    // // Install stdout exporter pipeline to be able to retrieve the collected spans.
    // let provider = sdktrace::TracerProvider::builder()
        // .with_batch_exporter(SpanExporter::default(), Tokio)
        // .build();

    // let provider = TracerProvider::builder()
        // .with_simple_exporter(SpanExporter::default())
        // .build();

    // global::set_tracer_provider(provider);

    // global::set_tracer_provider(provider.clone());
    // provider
// }

pub fn init_tracer(config: ServiceConfig) {
    match SpanExporter::new_tonic(ExportConfig::default(), TonicConfig::default()) {
        Ok(exporter) => {
            global::set_text_map_propagator(TraceContextPropagator::new());
            let provider = TracerProvider::builder()
                .with_simple_exporter(exporter)
                .build();
            global::set_tracer_provider(provider);
        },
        Err(why) => panic!("{:?}", why)
    }

}
