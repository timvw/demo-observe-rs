mod otel;

use anyhow::Result;
use axum::extract::State;
use axum::{http::StatusCode, routing::get, Router};
use http::header;
use http::HeaderMap;
use opentelemetry::global;
use opentelemetry_http::HeaderInjector;
use tokio::signal;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tracing::{info, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Clone, Debug)]
pub struct ServerState {
    http_client: reqwest::Client,
}

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

    let trace_layer = otel::build_otel_trace_layer();

    let server_state = ServerState {
        http_client: reqwest::Client::new(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/error", get(error))
        .layer(trace_layer)
        .layer(SetSensitiveRequestHeadersLayer::new(vec![
            header::AUTHORIZATION,
            header::COOKIE,
        ]))
        .with_state(server_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    meter_provider.shutdown()?;
    logger_provider.shutdown()?;
    tracer_provider.shutdown()?;

    Ok(())
}

fn build_request_headers() -> HeaderMap {
    let context = Span::current().context();
    let mut req_headers = HeaderMap::new();

    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&context, &mut HeaderInjector(&mut req_headers))
    });

    req_headers
}

async fn root(State(server_state): State<ServerState>) -> Result<&'static str, StatusCode> {
    let req_headers = build_request_headers();

    let health_rsp = server_state
        .http_client
        .get("http://localhost:3000/health")
        .headers(req_headers)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let body = health_rsp
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    info!("we received {}", body);

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
