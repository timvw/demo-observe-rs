# Demo observe-rs

Demonstrate observability with [OpenTelemetry](https://opentelemetry.io/) in [Rust](https://www.rust-lang.org/).

## Simple (hello world) application

```bash
cargo run --bin simple
```

## (Axum) server application

```bash
cargo run --bin server
```

## Guidelines

Use the [Tracing](https://github.com/tokio-rs/tracing) macros to instrument your application.
[Log](https://docs.rs/log/latest/log/) does not have the context which tracing has.
With the aid of [opentelemetry-appender-log](https://crates.io/crates/opentelemetry-appender-log) it's possible to retrofit libraries which already use Log.

