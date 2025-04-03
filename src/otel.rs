use opentelemetry::trace::TracerProvider as _;
// use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
use tracing_subscriber::{EnvFilter, prelude::*};

pub fn init_otel_traces(name: &str) {
    // #[allow(clippy::expect_used)]
    // let _exporter = SpanExporter::builder()
    //     .with_tonic()
    //     .with_endpoint("http://localhost:4317")
    //     .build()
    //     .expect("unable to setup exporter");

    let exporter = opentelemetry_stdout::SpanExporter::default();

    let resource = Resource::builder()
        .with_service_name(name.to_string())
        .build();

    let trace_provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(exporter)
        .build();

    let fmt_layer = tracing_subscriber::fmt::Layer::default();
    let tracer = trace_provider.tracer(name.to_string());

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        // NOTE: The actual Layer responsible for sending the data to OTEL.
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        // NOTE: And this (Layer) sends to stdout.
        .with(fmt_layer)
        .init();
    // NOTE: I can see traces without this... Need to investigate later.
    // global::set_tracer_provider(trace_provider);
}
