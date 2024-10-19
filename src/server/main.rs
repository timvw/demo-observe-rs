mod otel;

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::signal;

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

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/error", get(error));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    meter_provider.shutdown()?;
    logger_provider.shutdown()?;
    tracer_provider.shutdown()?;

    Ok(())
}

async fn root() -> Result<&'static str, StatusCode> {
    Ok("root")
}

async fn health() -> Result<&'static str, StatusCode> {
    Ok("UP")
}

async fn error() -> Result<&'static str, StatusCode> {
    Err(StatusCode::SERVICE_UNAVAILABLE)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
