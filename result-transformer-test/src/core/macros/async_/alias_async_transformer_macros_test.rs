use result_transformer::__internal::*;
use result_transformer::async_::macros::*;
#[allow(unused)]
use result_transformer::async_::{
    AsyncErrTransformer as _, AsyncOkTransformer as _, AsyncResultTransformer as _,
};
#[allow(unused)]
use result_transformer::sync::macros::*;

alias_async_ok_transformer!(pub trait AliasAsyncOk: [_ => _]);
alias_async_err_transformer!(pub trait AliasAsyncErr: [_ => _]);
alias_async_result_transformer!(pub trait AliasAsyncRes: [_, _ => _, _]);

struct AsyncDummy;

impl_async_ok_transformer!(
    impl_for = AsyncDummy,
    input_ok = i32,
    output_ok = i32,
    transform_ok = |x| async move { x * 2 },
);

impl_async_err_transformer!(
    impl_for = AsyncDummy,
    input_err = &'static str,
    output_err = String,
    transform_err = |e: &'static str| async move { e.to_string() },
);

impl_async_result_transformer_via_self_parts!(
    impl_for = AsyncDummy,
    input_ok = i32,
    input_err = &'static str,
);

#[tokio::test]
async fn async_alias_traits_compile() {
    fn assert_ok<T: AliasAsyncOk<i32, i32>>() {}
    fn assert_err<T: AliasAsyncErr<&'static str, String>>() {}
    fn assert_res<T: AliasAsyncRes<i32, &'static str, i32, String>>() {}
    assert_ok::<AsyncDummy>();
    assert_err::<AsyncDummy>();
    assert_res::<AsyncDummy>();
}
