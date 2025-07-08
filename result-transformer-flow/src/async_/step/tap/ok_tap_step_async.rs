use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncOkFlow;

/// Step that passes the success value to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct OkTapStepAsync<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<TapFn, InputOk, OutputOk> OkTapStepAsync<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    /// Creates a new [`OkTapStepAsync`].
    ///
    /// * `tap` - closure that processes the success value
    pub fn new(tap: TapFn) -> Self {
        Self {
            tap,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<TapFn, InputOk, OutputOk> AsyncOkFlow<InputOk> for OkTapStepAsync<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> Pin<Box<dyn Future<Output = OutputOk> + Send + Sync>> + Send + Sync,
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
{
    type OutputOk = OutputOk;

    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        (self.tap)(input_ok).await
    }
}
