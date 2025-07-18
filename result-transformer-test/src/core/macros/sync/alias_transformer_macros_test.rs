use result_transformer::sync::macros::*;
#[allow(unused)]
use result_transformer::sync::{ErrTransformer as _, OkTransformer as _, ResultTransformer as _};

alias_ok_transformer!(pub trait AliasOk: [_ => _]);
alias_err_transformer!(pub trait AliasErr: [_ => _]);
alias_result_transformer!(pub trait AliasRes: [_, _ => _, _]);

struct Dummy;

impl_ok_transformer!(
    impl_for = Dummy,
    input_ok = i32,
    output_ok = i64,
    transform_ok = |x| x as i64,
);

impl_err_transformer!(
    impl_for = Dummy,
    input_err = &'static str,
    output_err = String,
    transform_err = |e: &'static str| e.to_string(),
);

impl_result_transformer_via_self_parts!(
    impl_for = Dummy,
    input_ok = i32,
    input_err = &'static str,
);

#[test]
fn alias_traits_compile() {
    let d = Dummy;
    let _: &dyn AliasOk<i32, i64> = &d;
    let _: &dyn AliasErr<&'static str, String> = &d;
    let _: &dyn AliasRes<i32, &'static str, i64, String> = &d;
}
