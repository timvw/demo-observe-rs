use anyhow::{Context, Result};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::TonicExporterBuilder;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_sdk::runtime;
use std::time::SystemTime;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {

    std::env::set_var("OTEL_LOG_LEVEL", "info");
    std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://192.168.1.205:4317");
    std::env::set_var("OTEL_SERVICE_NAME", "demo-observe-rs");

    let logger_provider = init_logs()?;
    let now = SystemTime::now();
    let msg = format!("we are running hello world {:?}", now);
    log::info!("{}", &msg);
    println!("{}", &msg);
    logger_provider.shutdown()?;
    Ok(())
}

fn init_logs() -> Result<LoggerProvider> {
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
