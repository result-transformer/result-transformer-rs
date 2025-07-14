//! impl_transformer_macros_test.rs – Tests for synchronous transformer macros
//!
//! Located at `result-transformer-test/src/core/macros/sync/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features core-sync-macros -- core::macros::sync::impl_transformer_macros_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::sync::macros::*;

/// Brings `.transform()` into scope via trait implementation.
#[allow(unused_imports)]
use result_transformer::sync::{ErrTransformer as _, OkTransformer as _, ResultTransformer as _};

//
// ────────────────────────────────────────────────
// Impl: OkTransformer Variants
// ────────────────────────────────────────────────
//

#[allow(unused)]
struct OkViaInto;
impl_ok_transformer_via_input_into!(impl_for = OkViaInto, input_ok = i32, output_ok = i64,);

#[allow(unused)]
struct OkViaFrom;
impl_ok_transformer_via_output_from!(impl_for = OkViaFrom, input_ok = i32, output_ok = i64,);

//
// ────────────────────────────────────────────────
// Impl: ErrTransformer Variants
// ────────────────────────────────────────────────
//

#[allow(unused)]
struct ErrViaInto;
impl_err_transformer_via_input_into!(
    impl_for = ErrViaInto,
    input_err = &'static str,
    output_err = String,
);

#[allow(unused)]
struct ErrViaFrom;
impl_err_transformer_via_output_from!(
    impl_for = ErrViaFrom,
    input_err = &'static str,
    output_err = String,
);

//
// ────────────────────────────────────────────────
// Impl: ResultTransformer Variants
// ────────────────────────────────────────────────
//

#[allow(unused)]
struct ResultViaFn;
impl_result_transformer!(
    impl_for = ResultViaFn,
    input_ok = i32,
    input_err = &'static str,
    output_ok = i64,
    output_err = String,
    transform_result = |res: Result<i32, &'static str>| res.map(|o| (o as i64) * 2).map_err(|e| format!("E:{e}")),
);

#[allow(unused)]
struct ResultViaErrFn;
impl_result_transformer_via_err_transform_fn!(
    impl_for = ResultViaErrFn,
    input_ok = i32,
    input_err = &'static str,
    output_ok = i32,
    output_err = String,
    transform_err = |e| Err::<i32, _>(format!("E:{e}")),
);

#[allow(unused)]
struct ResultViaOkFn;
impl_result_transformer_via_ok_transform_fn!(
    impl_for = ResultViaOkFn,
    input_ok = i32,
    input_err = &'static str,
    output_ok = String,
    output_err = &'static str,
    transform_ok = |o: i32| Ok::<String, _>(o.to_string()),
);

//
// ────────────────────────────────────────────────
// Impl: Composite Transformer (via parts)
// ────────────────────────────────────────────────
//

struct SyncTransformer;

impl_ok_transformer!(
    impl_for = SyncTransformer,
    input_ok = i32,
    output_ok = i32,
    transform_ok = |ok| ok * 2
);

impl_err_transformer!(
    impl_for = SyncTransformer,
    input_err = &'static str,
    output_err = String,
    transform_err = |err| format!("E:{}", err)
);

impl_result_transformer_via_self_parts!(
    impl_for = SyncTransformer,
    input_ok = i32,
    input_err = &'static str
);

//
// ────────────────────────────────────────────────
// Tests: OK
// ────────────────────────────────────────────────
//

#[test]
fn transforms_ok_value() {
    let transformer = SyncTransformer;
    let result = transformer.transform(Ok(3));
    assert_eq!(result, Ok(6));
}

#[test]
fn ok_via_into_works() {
    assert_eq!(OkViaInto.transform_ok(1), 1i64);
}

#[test]
fn ok_via_from_works() {
    assert_eq!(OkViaFrom.transform_ok(2), 2i64);
}

//
// ────────────────────────────────────────────────
// Tests: ERR
// ────────────────────────────────────────────────
//

#[test]
fn transforms_err_value() {
    let transformer = SyncTransformer;
    let result = transformer.transform(Err("oops"));
    assert_eq!(result, Err("E:oops".to_string()));
}

#[test]
fn err_via_into_works() {
    assert_eq!(ErrViaInto.transform_err("a"), "a".to_string());
}

#[test]
fn err_via_from_works() {
    assert_eq!(ErrViaFrom.transform_err("b"), "b".to_string());
}

//
// ────────────────────────────────────────────────
// Tests: RESULT
// ────────────────────────────────────────────────
//

#[test]
fn result_via_fn_works() {
    let t = ResultViaFn;
    assert_eq!(t.transform(Ok(3)), Ok(6));
    assert_eq!(t.transform(Err("x")), Err("E:x".to_string()));
}

#[test]
fn result_via_err_fn_works() {
    let t = ResultViaErrFn;
    assert_eq!(t.transform(Ok(4)), Ok(4));
    assert_eq!(t.transform(Err("y")), Err("E:y".to_string()));
}

#[test]
fn result_via_ok_fn_works() {
    let t = ResultViaOkFn;
    assert_eq!(t.transform(Ok(5)), Ok("5".to_string()));
    assert_eq!(t.transform(Err("bad")), Err("bad"));
}
