mod alias_transformer_shared;

#[cfg(feature = "core-sync")]
pub mod alias_err_transformer;
#[cfg(feature = "core-sync")]
pub mod alias_ok_transformer;
#[cfg(feature = "core-sync")]
pub mod alias_result_transformer;

#[cfg(feature = "core-async")]
pub mod alias_async_err_transformer;
#[cfg(feature = "core-async")]
pub mod alias_async_ok_transformer;
#[cfg(feature = "core-async")]
pub mod alias_async_result_transformer;
