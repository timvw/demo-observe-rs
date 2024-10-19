use anyhow::{Context, Result};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::TonicExporterBuilder;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_sdk::runtime;
use std::time::{Duration, SystemTime};
use std::str::FromStr;
use opentelemetry::global;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::metrics::reader::{DefaultAggregationSelector, DefaultTemporalitySelector};
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::propagation::{BaggagePropagator, TraceContextPropagator};
use opentelemetry_sdk::trace::{BatchConfig, Config};
use tracing::{debug, info, instrument};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() -> Result<()> {

    std::env::set_var("RUST_LOG", "demo_observe_rs=debug");
    std::env::set_var("OTEL_LOG_LEVEL", "debug");
    std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://192.168.1.205:4317");
    std::env::set_var("OTEL_SERVICE_NAME", "demo-observe-rs");

    let tracer_provider = init_tracer_provider()?;
    demonstrate_tracing();

    let logger_provider = init_logger_provider()?;
    demonstrate_logging();

    let meter_provider = init_meter_provider()?;
    demonstrate_metrics().await;

    meter_provider.shutdown()?;
    logger_provider.shutdown()?;
    tracer_provider.shutdown()?;
    Ok(())
}

#[instrument]
fn demonstrate_tracing() {
    info!("demonstrating tracer");
    debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
    tracing_nested();
    info!("we are done running the nested call");
}

#[instrument]
fn tracing_nested() {
    info!("tracing nested")
}

async fn demonstrate_metrics() {
    let meter = global::meter("");
    let counter = meter.u64_counter("my_counter").init();
    for _ in 1..5 {
        let msg = "incrementing counter with 1";
        log::info!("{}", &msg);
        info!("{}", &msg);
        counter.add(1, &[]);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

fn demonstrate_logging() {
    let now = SystemTime::now();
    let msg = format!("we are running hello world {:?}", now);
    log::info!("{}", &msg);
    info!("{}", &msg);
}

fn init_logger_provider() -> Result<LoggerProvider> {
    let logger_provider = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(TonicExporterBuilder::default())
        .install_batch(runtime::Tokio)
        .with_context(|| "Failed to create OTEL LoggerProvider".to_string())?;

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender))
        .with_context(|| "Failed to set global OTEL logger")?;
    let otel_log_level = std::env::var("OTEL_LOG_LEVEL").unwrap_or("info".to_string());
    let log_level =log::Level::from_str(&otel_log_level).with_context(|| format!("Failed to parse {} as log::Level", &otel_log_level))?;
    log::set_max_level(log_level.to_level_filter());

    Ok(logger_provider)
}

fn init_meter_provider() -> Result<SdkMeterProvider> {
    let aggregation_selector = Box::new(DefaultAggregationSelector::new());
    let temporality_selector = Box::new(DefaultTemporalitySelector::new());
    let exporter = TonicExporterBuilder::default()
        .build_metrics_exporter(aggregation_selector, temporality_selector)?;
    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(Duration::from_secs(10))
        .build();
    let provider = SdkMeterProvider::builder().with_reader(reader).build();
    global::set_meter_provider(provider.clone());
    Ok(provider)
}

fn init_tracer_provider() -> Result<opentelemetry_sdk::trace::TracerProvider> {
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(TonicExporterBuilder::default())
        .with_trace_config(Config::default())
        .with_batch_config(BatchConfig::default())
        .install_batch(runtime::Tokio)?;
    global::set_tracer_provider(tracer_provider.clone());

    let baggage_propagator = BaggagePropagator::new();
    let trace_context_propagator = TraceContextPropagator::new();

    // Then create a composite propagator
    let propagator = TextMapCompositePropagator::new(vec![
        Box::new(baggage_propagator),
        Box::new(trace_context_propagator),
    ]);

    global::set_text_map_propagator(propagator);

    let otel_tracing_layer = tracing_opentelemetry::layer()
        .with_error_records_to_exceptions(true)
        .with_tracer(tracer_provider.tracer("demo-app"));

    let subscriber = tracing_subscriber::registry()
        .with(otel_tracing_layer)
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_timer(tracing_subscriber::fmt::time::uptime()),
        );
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(tracer_provider)
}
