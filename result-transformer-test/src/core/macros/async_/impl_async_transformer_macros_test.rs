//! impl_async_transformer_macros_test.rs – Tests for asynchronous transformer macros
//!
//! Located at `result-transformer-test/src/core/macros/async_/`.
//! ────────────────────────────────────────────────────────────────
//!  HOW TO RUN
//!  $ cargo test --package result-transformer-test --lib --features core-async-macros -- core::macros::async_::impl_async_transformer_macros_test --show-output
//! ────────────────────────────────────────────────────────────────

use result_transformer::async_::macros::*;
use result_transformer::sync::macros::*;

/// Brings transformer traits into scope for `.transform_async()` etc.
#[allow(unused_imports)]
use result_transformer::async_::{
    AsyncErrTransformer as _, AsyncOkTransformer as _, AsyncResultTransformer as _,
};

//
// ────────────────────────────────────────────────
// Impl: AsyncOkTransformer Variants
// ────────────────────────────────────────────────
//

#[allow(unused)]
struct AsyncOkViaInto;
impl_async_ok_transformer_via_input_into!(
    impl_for = AsyncOkViaInto,
    input_ok = i32,
    output_ok = i64,
);

#[allow(unused)]
struct AsyncOkViaFrom;
impl_async_ok_transformer_via_output_from!(
    impl_for = AsyncOkViaFrom,
    input_ok = i32,
    output_ok = i64,
);

struct AsyncOkViaSelf;
impl_ok_transformer!(
    impl_for = AsyncOkViaSelf,
    input_ok = i32,
    output_ok = i32,
    transform_ok = |x| x + 1,
);
impl_async_ok_transformer_via_self_ok_transformer!(
    impl_for = AsyncOkViaSelf,
    impl_via = AsyncOkViaSelf,
    input_ok = i32,
);

//
// ────────────────────────────────────────────────
// Impl: AsyncErrTransformer Variants
// ────────────────────────────────────────────────
//

#[allow(unused)]
struct AsyncErrViaInto;
impl_async_err_transformer_via_input_into!(
    impl_for = AsyncErrViaInto,
    input_err = &'static str,
    output_err = String,
);

#[allow(unused)]
struct AsyncErrViaFrom;
impl_async_err_transformer_via_output_from!(
    impl_for = AsyncErrViaFrom,
    input_err = &'static str,
    output_err = String,
);

struct AsyncErrViaSelf;
impl_err_transformer!(
    impl_for = AsyncErrViaSelf,
    input_err = &'static str,
    output_err = String,
    transform_err = |err| format!("E:{err}"),
);
impl_async_err_transformer_via_self_err_transformer!(
    impl_for = AsyncErrViaSelf,
    impl_via = AsyncErrViaSelf,
    input_err = &'static str,
);

//
// ────────────────────────────────────────────────
// Impl: AsyncResultTransformer Variants
// ────────────────────────────────────────────────
//

struct AsyncTransformer;
impl_async_ok_transformer!(
    impl_for = AsyncTransformer,
    input_ok = i32,
    output_ok = i32,
    transform_ok = |ok| async move { ok * 2 }
);
impl_async_err_transformer!(
    impl_for = AsyncTransformer,
    input_err = &'static str,
    output_err = String,
    transform_err = |err| async move { format!("E:{}", err) }
);
impl_async_result_transformer_via_self_parts!(
    impl_for = AsyncTransformer,
    input_ok = i32,
    input_err = &'static str
);

#[allow(unused)]
struct AsyncResultViaFn;
impl_async_result_transformer!(
    impl_for = AsyncResultViaFn,
    input_ok = i32,
    input_err = &'static str,
    output_ok = i64,
    output_err = String,
    transform_result = |res :Result<i32, &'static str>| async move {
        res.map(|ok| (ok as i64) * 2).map_err(|err| format!("E:{err}"))
    },
);

#[allow(unused)]
struct AsyncResultViaErrFn;
impl_async_result_transformer_via_err_transform_fn!(
    impl_for = AsyncResultViaErrFn,
    input_ok = i32,
    input_err = &'static str,
    output_ok = i32,
    output_err = String,
    transform_err = |err| async move { Err::<i32, _>(format!("E:{err}")) },
);

#[allow(unused)]
struct AsyncResultViaOkFn;
impl_async_result_transformer_via_ok_transform_fn!(
    impl_for = AsyncResultViaOkFn,
    input_ok = i32,
    input_err = &'static str,
    output_ok = String,
    output_err = &'static str,
    transform_ok = |ok: i32| async move { Ok::<String, _>(ok.to_string()) },
);

struct AsyncViaSelfResult;
impl_result_transformer!(
    impl_for = AsyncViaSelfResult,
    input_ok = i32,
    input_err = &'static str,
    output_ok = i32,
    output_err = String,
    transform_result = |res: Result<i32, &'static str>| res.map(|o| o * 2).map_err(|e| format!("E:{e}")),
);
impl_async_result_transformer_via_self_result_transformer!(
    impl_for = AsyncViaSelfResult,
    input_ok = i32,
    input_err = &'static str,
);

//
// ────────────────────────────────────────────────
// Tests: OK
// ────────────────────────────────────────────────
//

#[tokio::test]
async fn async_transforms_ok_value() {
    let transformer = AsyncTransformer;
    let result = transformer.transform_async(Ok(3)).await;
    assert_eq!(result, Ok(6));
}

#[tokio::test]
async fn async_ok_via_into() {
    assert_eq!(AsyncOkViaInto.transform_ok_async(1).await, 1i64);
}

#[tokio::test]
async fn async_ok_via_from() {
    assert_eq!(AsyncOkViaFrom.transform_ok_async(2).await, 2i64);
}

#[tokio::test]
async fn async_ok_via_self() {
    assert_eq!(AsyncOkViaSelf.transform_ok_async(3).await, 4);
}

//
// ────────────────────────────────────────────────
// Tests: ERR
// ────────────────────────────────────────────────
//

#[tokio::test]
async fn async_transforms_err_value() {
    let transformer = AsyncTransformer;
    let result = transformer.transform_async(Err("oops")).await;
    assert_eq!(result, Err("E:oops".to_string()));
}

#[tokio::test]
async fn async_err_via_into() {
    assert_eq!(
        AsyncErrViaInto.transform_err_async("a").await,
        "a".to_string()
    );
}

#[tokio::test]
async fn async_err_via_from() {
    assert_eq!(
        AsyncErrViaFrom.transform_err_async("b").await,
        "b".to_string()
    );
}

#[tokio::test]
async fn async_err_via_self() {
    assert_eq!(
        AsyncErrViaSelf.transform_err_async("x").await,
        "E:x".to_string()
    );
}

//
// ────────────────────────────────────────────────
// Tests: RESULT
// ────────────────────────────────────────────────
//

#[tokio::test]
async fn async_result_via_fn() {
    let t = AsyncResultViaFn;
    assert_eq!(t.transform_async(Ok(2)).await, Ok(4));
    assert_eq!(t.transform_async(Err("m")).await, Err("E:m".to_string()));
}

#[tokio::test]
async fn async_result_via_err_fn() {
    let t = AsyncResultViaErrFn;
    assert_eq!(t.transform_async(Ok(5)).await, Ok(5));
    assert_eq!(t.transform_async(Err("z")).await, Err("E:z".to_string()));
}

#[tokio::test]
async fn async_result_via_ok_fn() {
    let t = AsyncResultViaOkFn;
    assert_eq!(t.transform_async(Ok(7)).await, Ok("7".to_string()));
    assert_eq!(t.transform_async(Err("bad")).await, Err("bad"));
}

#[tokio::test]
async fn async_result_via_self_result() {
    let t = AsyncViaSelfResult;
    assert_eq!(t.transform_async(Ok(6)).await, Ok(12));
    assert_eq!(t.transform_async(Err("q")).await, Err("E:q".to_string()));
}
