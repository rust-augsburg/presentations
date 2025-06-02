//! The code in this module basically is the example found at
//! https://github.com/tokio-rs/tracing-opentelemetry/blob/v0.1.x/examples/opentelemetry-otlp.rs

use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_sdk::{
    Resource,
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider},
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
};
use opentelemetry_semantic_conventions::{
    SCHEMA_URL,
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Resources will help you categorize telemetry data.
/// They might contain information about your kubernetes cluster, pod information and anything else
/// you would want to attach to your data (a service ID maybe to distinguish different instances of a service etc).
///
/// Creates a OTEL [`Resource`].
fn resource() -> Resource {
    Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_schema_url(
            [
                KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
            ],
            SCHEMA_URL,
        )
        .build()
}

/// The [`SdkMeterProvider`] collects and exports metrics data (counters, gauges, etc). Here we can
/// configure the transmission protocol, transmission intervals and much more.
fn init_meter_provider() -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::default())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    // Useful for dev-time: See everything in the terminal
    let stdout_reader =
        PeriodicReader::builder(opentelemetry_stdout::MetricExporter::default()).build();

    // We return the provider. The handle can be useful during a graceful shutdown.
    MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        .with_reader(stdout_reader)
        .build()
}

// Construct TracerProvider for OpenTelemetryLayer
fn init_tracer_provider() -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

/// Initializes a tracing subscriber that
/// - collects and sends out traces
/// - collects and sends out metrics
/// - logs to stdout
///
/// Returns an [`OtelGuard`] that holds the handles to the metrics and the trace providers.
pub fn init_tracing_subscriber() -> OtelGuard {
    let tracer_provider = init_tracer_provider();
    let meter_provider = init_meter_provider();

    global::set_meter_provider(meter_provider.clone());
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());
    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    tracing_subscriber::registry()
        // The global level filter prevents the exporter network stack
        // from reentering the globally installed OpenTelemetryLayer with
        // its own spans while exporting, as the libraries should not use
        // tracing levels below DEBUG. If the OpenTelemetry layer needs to
        // trace spans and events with higher verbosity levels, consider using
        // per-layer filtering to target the telemetry layer specifically,
        // e.g. by target matching.
        .with(tracing_subscriber::filter::EnvFilter::try_from_default_env().unwrap_or_default())
        .with(tracing_subscriber::fmt::layer().with_line_number(true))
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    OtelGuard {
        tracer_provider,
        meter_provider,
    }
}

/// Holds handles to tracing and metric providers.
/// They can be used to perform any kind of cleanup operations when the program shuts down.
pub struct OtelGuard {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}
