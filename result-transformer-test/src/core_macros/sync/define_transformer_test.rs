//! define_transformer_test.rs – Tests for synchronous transformer macros
//!
//! Located at `result-transformer-test/src/core_macros/sync/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features core-sync-macros -- core_macros::sync::define_transformer_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::sync::macros::*;

struct SyncTransformer;

define_ok_transformer!(
    impl_for = SyncTransformer,
    input_ok = i32,
    output_ok = i32,
    transform_ok = |ok| ok * 2
);

define_err_transformer!(
    impl_for = SyncTransformer,
    input_err = &'static str,
    output_err = String,
    transform_err = |err| format!("E:{}", err)
);

impl_result_transformer_from_parts!(impl_for = SyncTransformer, input_ok = i32, input_err = &'static str);

/// Brings `transform_sync` into scope via trait implementation.
/// The trait is used implicitly, so we alias it as `_` to avoid unused import warnings.
#[allow(unused_imports)]
use result_transformer::sync::ResultTransformer as _;

#[test]
fn transforms_ok_value() {
    let transformer = SyncTransformer;
    let result = transformer.transform(Ok(3));
    assert_eq!(result, Ok(6));
}

#[test]
fn transforms_err_value() {
    let transformer = SyncTransformer;
    let result = transformer.transform(Err("oops"));
    assert_eq!(result, Err("E:oops".to_string()));
}
