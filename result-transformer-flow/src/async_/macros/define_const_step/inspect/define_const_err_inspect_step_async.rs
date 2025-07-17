/// Defines a const [`ErrInspectStepAsync`].
///
/// Shorthand syntax: `($name, $err_type, $inspector)`.
///
/// # Parameters
/// - `name` - Identifier for the constant.
/// - `err_type` - Error type inspected by the asynchronous function.
/// - `inspector` - Asynchronous function receiving a reference to the error value.
#[macro_export]
macro_rules! define_const_err_inspect_step_async {
    (
        name = $name:ident,
        err_type = $err_type:ty,
        inspector = $inspector:expr $(,)?
    ) => {
        const $name: result_transformer::flow::async_::step::ErrInspectStepAsync<
            $err_type,
            fn(&$err_type) -> std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
            std::pin::Pin<Box<dyn core::future::Future<Output = ()> + Send>>,
        > = result_transformer::flow::async_::step::ErrInspectStepAsync::new($inspector);
    };

    (
        $name:ident,
        $err_type:ty,
        $inspector:expr $(,)?
    ) => {
        result_transformer::flow::async_::macros::define_const_err_inspect_step_async!(
            name = $name,
            err_type = $err_type,
            inspector = $inspector
        );
    };
}
pub use define_const_err_inspect_step_async;
