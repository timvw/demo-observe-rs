use anyhow::{Context, Result};
use axum::body::Body;
use http::Request;
use opentelemetry::{
	global,
	propagation::TextMapCompositePropagator,
	trace::{TraceContextExt, Tracer, TracerProvider},
};
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_http::HeaderExtractor;
use opentelemetry_otlp::TonicExporterBuilder;
use opentelemetry_sdk::{
	logs::LoggerProvider,
	metrics::{
		reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
		PeriodicReader, SdkMeterProvider,
	},
	propagation::{BaggagePropagator, TraceContextPropagator},
	runtime,
	trace::{BatchConfig, Config},
};
use std::{str::FromStr, time::Duration};
use tower_http::trace::{DefaultMakeSpan, HttpMakeClassifier, MakeSpan, TraceLayer};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn init_logger_provider() -> Result<LoggerProvider> {
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
	let log_level = log::Level::from_str(&otel_log_level)
		.with_context(|| format!("Failed to parse {} as log::Level", &otel_log_level))?;
	log::set_max_level(log_level.to_level_filter());

	Ok(logger_provider)
}

pub fn init_meter_provider() -> Result<SdkMeterProvider> {
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

pub fn init_tracer_provider() -> Result<opentelemetry_sdk::trace::TracerProvider> {
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

pub fn build_otel_trace_layer() -> TraceLayer<HttpMakeClassifier, fn(&Request<Body>) -> Span> {
	let span_builder = |req: &Request<Body>| {
		let parent_context = global::get_text_map_propagator(|propagator| {
			propagator.extract(&HeaderExtractor(req.headers()))
		});

		let otel_tracer = global::tracer("");
		let otel_span_name = format!("{} {}", req.method(), req.uri().path());
		let otel_span = otel_tracer.start_with_context(otel_span_name, &parent_context);

		let cx = opentelemetry::Context::current_with_span(otel_span);

		let span = DefaultMakeSpan::default().make_span(req);
		span.set_parent(cx);
		span
	};

	TraceLayer::new_for_http().make_span_with(span_builder)
}
