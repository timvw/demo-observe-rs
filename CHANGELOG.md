# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/timvw/demo-observe-rs/releases/tag/v0.1.0) - 2024-10-26

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

- *(deps)* update opentelemetry-rust monorepo to 0.26
- *(deps)* update rust crate anyhow to v1.0.91

### Other

- do not publish a release on the registry
- add changelog.md file
- update code to fit with new api in otel
- allow merges
- Merge pull request [#8](https://github.com/timvw/demo-observe-rs/pull/8) from timvw/renovate/serde-monorepo
- Merge pull request [#4](https://github.com/timvw/demo-observe-rs/pull/4) from timvw/renovate/tracing-opentelemetry-0.x
- add workflows
- fmt with cargo +nightly fmt
- fmt guidelines
- remove use of reqwest middleware as it does not help with our version
- move trace layer to otel
- update all libs
- clippy
- rename method
- add some entry
