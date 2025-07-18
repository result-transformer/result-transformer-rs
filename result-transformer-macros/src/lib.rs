//! Procedural macros for the `result-transformer` crates.

use proc_macro::TokenStream;

//core
mod core;

//alias_transformer

#[cfg(feature = "core-sync")]
#[proc_macro]
pub fn alias_err_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_err_transformer::expand(item)
}
#[cfg(feature = "core-sync")]
#[proc_macro]
pub fn alias_ok_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_ok_transformer::expand(item)
}
#[cfg(feature = "core-sync")]
#[proc_macro]
pub fn alias_result_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_result_transformer::expand(item)
}

//alias_async_transformer
#[cfg(feature = "core-async")]
#[proc_macro]
pub fn alias_async_err_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_async_err_transformer::expand(item)
}
#[cfg(feature = "core-async")]
#[proc_macro]
pub fn alias_async_ok_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_async_ok_transformer::expand(item)
}
#[cfg(feature = "core-async")]
#[proc_macro]
pub fn alias_async_result_transformer(item: TokenStream) -> TokenStream {
    core::alias_transformer::alias_async_result_transformer::expand(item)
}
