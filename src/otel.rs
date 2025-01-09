use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{trace as sdktrace, Resource};
use tracing_subscriber::prelude::*;

pub fn init_otel_traces(name: &str) {
    #[allow(clippy::expect_used)]
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()
        .expect("unable to setup exporter");
    let trace_provider = sdktrace::TracerProvider::builder()
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            name.to_string(),
        )]))
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build();

    let fmt_layer = tracing_subscriber::fmt::Layer::default();
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(trace_provider.tracer(name.to_string())))
        .with(fmt_layer)
        .init();
    // NOTE: I can see traces without this... Need to investigate later.
    // global::set_tracer_provider(trace_provider);
}
