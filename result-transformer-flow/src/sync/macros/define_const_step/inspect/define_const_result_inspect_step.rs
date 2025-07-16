#[macro_export]
macro_rules! define_const_result_inspect_step {
    (
        name = $name:ident,
        ok_type = $ok_type:ty,
        err_type = $err_type:ty,
        inspector = $inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::sync::step::ResultInspectStep<
            $ok_type,
            $err_type,
            fn(&Result<$ok_type, $err_type>),
        > = result_transformer::flow::sync::step::ResultInspectStep::new($inspector);
    };

    (
        $name:ident,
        [$ok_type:ty, $err_type:ty],
        $inspector:expr $(,)?
    ) => {
        result_transformer::flow::sync::macros::define_const_result_inspect_step!(
            name = $name,
            ok_type = $ok_type,
            err_type = $err_type,
            inspector = $inspector
        );
    };
}
pub use define_const_result_inspect_step;
