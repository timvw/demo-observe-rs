mod otel;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // better set these through configuration...
    std::env::set_var("RUST_LOG", "demo_observe_rs=debug");
    std::env::set_var("OTEL_LOG_LEVEL", "debug");
    std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://192.168.1.205:4317");
    std::env::set_var("OTEL_SERVICE_NAME", "demo-observe-rs");

    let tracer_provider = otel::init_tracer_provider()?;
    let logger_provider = otel::init_logger_provider()?;
    let meter_provider = otel::init_meter_provider()?;

    meter_provider.shutdown()?;
    logger_provider.shutdown()?;
    tracer_provider.shutdown()?;

    Ok(())
}
