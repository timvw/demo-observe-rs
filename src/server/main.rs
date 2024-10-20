mod otel;

use anyhow::Result;
use axum::body::Body;
use axum::{http::StatusCode, routing::get, Router};
use http::Request;
use opentelemetry::trace::{TraceContextExt, Tracer};
use opentelemetry::{global, Context};
use opentelemetry_http::HeaderExtractor;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[tokio::main]
async fn main() -> Result<()> {
    // better set these through configuration...
    std::env::set_var("RUST_LOG", "info,tower_http=debug,demo_observe_rs=debug");
    std::env::set_var("OTEL_LOG_LEVEL", "debug");
    std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", "http://192.168.1.205:4317");
    std::env::set_var("OTEL_SERVICE_NAME", "demo-observe-rs");

    let tracer_provider = otel::init_tracer_provider()?;
    let logger_provider = otel::init_logger_provider()?;
    let meter_provider = otel::init_meter_provider()?;

    let on_request_handler = |req: &Request<Body>, span: &Span| {
        let parent_context = global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(req.headers()))
        });
        let otel_tracer = global::tracer("");
        let otel_span_name = format!("{} {}", req.method(), req.uri().path());
        let otel_span = otel_tracer.start_with_context(otel_span_name, &parent_context);
        let cx = Context::current_with_span(otel_span);
        span.set_parent(cx);
    };

    let trace_layer = TraceLayer::new_for_http().on_request(on_request_handler);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/error", get(error))
        .layer(trace_layer);

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
