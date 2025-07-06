# Result Transformer

This workspace contains several crates that provide traits and utilities for transforming `Result` values.

- **result-transformer** – reexports the core traits, macros and optional flow utilities.
- **result-transformer-core** – definitions for synchronous and asynchronous transformers.
- **result-transformer-macros** – procedural macros for implementing the transformer traits.
- **result-transformer-flow** – helper types for building transformation pipelines.
- **result-transformer-derive** – proc-macro crate reserved for derives (currently empty).
- **result-transformer-dependencies** – exposes optional third-party dependencies behind feature flags.
- **result-transformer-test** – integration tests used for verifying the library and macro crates.
