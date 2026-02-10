# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/timvw/demo-observe-rs/releases/tag/v0.1.0) - 2026-02-10

### Added

- leverage OTEL_METRIC_EXPORT_INTERVAL instead of hardcoding
- introduce support for a potential .env file as well
- introduce settings
- exclude some headers from being logged
- demonstrate context propagation
- configure tower trace layer to be wrapped by an otel trace which shows some extra info
- add tower-http tracing
- add basic axum server
- add a server application and document some guidelines
- demonstrate tracing
- demonstrate meter
- enhance logging
- initial example with logging to otlp endpoint

### Fixed

- *(deps)* update rust crate axum to 0.8 ([#31](https://github.com/timvw/demo-observe-rs/pull/31))
- *(deps)* update rust crate tracing-subscriber to v0.3.20 [security] ([#34](https://github.com/timvw/demo-observe-rs/pull/34))
- *(deps)* update rust crate tokio to v1.43.1 [security] ([#33](https://github.com/timvw/demo-observe-rs/pull/33))
- *(deps)* update rust crate log to v0.4.28 ([#32](https://github.com/timvw/demo-observe-rs/pull/32))
- *(deps)* update rust crate serde to v1.0.226 ([#29](https://github.com/timvw/demo-observe-rs/pull/29))
- *(deps)* update rust crate http to v1.3.1 ([#27](https://github.com/timvw/demo-observe-rs/pull/27))
- *(deps)* update rust crate anyhow to v1.0.100 ([#26](https://github.com/timvw/demo-observe-rs/pull/26))
- *(deps)* update rust crate tracing to v0.1.41 ([#24](https://github.com/timvw/demo-observe-rs/pull/24))
- *(deps)* update rust crate tower-http to v0.6.2 ([#22](https://github.com/timvw/demo-observe-rs/pull/22))
- *(deps)* update rust crate axum to v0.7.9 ([#21](https://github.com/timvw/demo-observe-rs/pull/21))
- *(deps)* update rust crate axum to v0.7.8 ([#20](https://github.com/timvw/demo-observe-rs/pull/20))
- *(deps)* update rust crate clap to v4.5.21 ([#18](https://github.com/timvw/demo-observe-rs/pull/18))
- *(deps)* update rust crate serde to v1.0.215 ([#16](https://github.com/timvw/demo-observe-rs/pull/16))
- *(deps)* update rust crate tokio to v1.41.1 ([#15](https://github.com/timvw/demo-observe-rs/pull/15))
- *(deps)* update rust crate anyhow to v1.0.93
- *(deps)* update rust crate anyhow to v1.0.92
- *(deps)* update rust crate serde to v1.0.214
- *(deps)* update rust crate reqwest to v0.12.9
- *(deps)* update rust crate tokio to v1.41.0
- *(deps)* update opentelemetry-rust monorepo to 0.26
- *(deps)* update rust crate anyhow to v1.0.91

### Other

- *(deps)* bump rustls from 0.23.15 to 0.23.18 ([#23](https://github.com/timvw/demo-observe-rs/pull/23))
- *(deps)* update rust crate clap to v4.5.53 ([#28](https://github.com/timvw/demo-observe-rs/pull/28))
- do not publish a release on the registry
- add changelog.md file
- update code to fit with new api in otel
- allow merges
- Merge pull request #8 from timvw/renovate/serde-monorepo
- Merge pull request #4 from timvw/renovate/tracing-opentelemetry-0.x
- add workflows
- fmt with cargo +nightly fmt
- fmt guidelines
- remove use of reqwest middleware as it does not help with our version
- move trace layer to otel
- update all libs
- clippy
- rename method
- add some entry
