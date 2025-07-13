use result_transformer::__internal::*;
use result_transformer::async_::macros::*;

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

impl_async_result_transformer_via_self_parts!(impl_for = AsyncTransformer, input_ok = i32, input_err = &'static str);

/// Brings `transform_async` into scope via trait implementation.
/// The trait is used implicitly, so we alias it as `_` to avoid unused import warnings.
#[allow(unused_imports)]
use result_transformer::async_::AsyncResultTransformer as _;

#[tokio::test]
async fn async_transforms_ok_value() {
    let transformer = AsyncTransformer;
    let result = transformer.transform_async(Ok(3)).await;
    assert_eq!(result, Ok(6));
}

#[tokio::test]
async fn async_transforms_err_value() {
    let transformer = AsyncTransformer;
    let result = transformer.transform_async(Err("oops")).await;
    assert_eq!(result, Err("E:oops".to_string()));
}
