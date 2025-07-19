//! Procedural macros for the `result-transformer` crates.

#[cfg(any(feature = "core-sync", feature = "core-async"))]
use proc_macro::TokenStream;

//core
mod core;

//alias_transformer

#[cfg(feature = "core-sync")]
/// Creates a trait alias for [`ErrTransformer`].
///
/// Use the syntax `(pub trait AliasName: [InputErr => OutputErr])`.
/// `InputErr` and `OutputErr` may be `_` to generate a generic parameter.
#[proc_macro]
pub fn alias_err_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_err_transformer::expand(item)
}
#[cfg(feature = "core-sync")]
/// Creates a trait alias for [`OkTransformer`].
///
/// Use the syntax `(pub trait AliasName: [InputOk => OutputOk])`.
/// `InputOk` and `OutputOk` may be `_` to generate a generic parameter.
#[proc_macro]
pub fn alias_ok_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_ok_transformer::expand(item)
}
#[cfg(feature = "core-sync")]
/// Creates a trait alias for [`ResultTransformer`].
///
/// Use the syntax `(pub trait AliasName: [InputOk, InputErr => OutputOk, OutputErr])`.
/// Type positions may use `_` to insert generic parameters.
#[proc_macro]
pub fn alias_result_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_result_transformer::expand(item)
}

//alias_async_transformer
#[cfg(feature = "core-async")]
/// Creates a trait alias for [`AsyncErrTransformer`].
///
/// Use the syntax `(pub trait AliasName: [InputErr => OutputErr])`.
/// `InputErr` and `OutputErr` may be `_` to generate a generic parameter.
#[proc_macro]
pub fn alias_async_err_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_async_err_transformer::expand(item)
}
#[cfg(feature = "core-async")]
/// Creates a trait alias for [`AsyncOkTransformer`].
///
/// Use the syntax `(pub trait AliasName: [InputOk => OutputOk])`.
/// `InputOk` and `OutputOk` may be `_` to generate a generic parameter.
#[proc_macro]
pub fn alias_async_ok_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_async_ok_transformer::expand(item)
}
#[cfg(feature = "core-async")]
/// Creates a trait alias for [`AsyncResultTransformer`].
///
/// Use the syntax `(pub trait AliasName: [InputOk, InputErr => OutputOk, OutputErr])`.
/// Type positions may use `_` to insert generic parameters.
#[proc_macro]
pub fn alias_async_result_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_async_result_transformer::expand(item)
}
