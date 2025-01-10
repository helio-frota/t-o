use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{trace as sdktrace, Resource};
use tracing_subscriber::{prelude::*, EnvFilter};

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
    let tracer = trace_provider.tracer(name.to_string());

    // let filter_otel = EnvFilter::new("info")
    //     .add_directive("hyper=off".parse().unwrap_or("off"))
    //     .add_directive("opentelemetry=off".parse().unwrap())
    //     .add_directive("tonic=off".parse().unwrap())
    //     .add_directive("h2=off".parse().unwrap())
    //     .add_directive("reqwest=off".parse().unwrap());

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
