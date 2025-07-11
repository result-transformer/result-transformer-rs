use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncOkFlow;

/// Step that passes the success value to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct OkTapStepAsync<InputOk, OutputOk, TapFn, TapFut>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    TapFn: Fn(InputOk) -> TapFut + Send + Sync,
    TapFut: Future<Output = OutputOk> + Send,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<InputOk, OutputOk, TapFn, TapFut> OkTapStepAsync<InputOk, OutputOk, TapFn, TapFut>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    TapFn: Fn(InputOk) -> TapFut + Send + Sync,
    TapFut: Future<Output = OutputOk> + Send,
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

impl<InputOk, OutputOk, TapFn, TapFut> AsyncOkFlow<InputOk>
    for OkTapStepAsync<InputOk, OutputOk, TapFn, TapFut>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    TapFn: Fn(InputOk) -> TapFut + Send + Sync,
    TapFut: Future<Output = OutputOk> + Send,
{
    type OutputOk = OutputOk;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        (self.tap)(input_ok)
    }
}
