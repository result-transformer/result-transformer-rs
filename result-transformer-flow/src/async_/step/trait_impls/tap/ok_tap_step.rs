//! Async implementation of ok tap step.

use crate::__internal::shared_step::OkTapStep;
use crate::async_::AsyncOkFlow;

impl<InputOk, OutputOk, TapFn> AsyncOkFlow<InputOk> for OkTapStep<InputOk, OutputOk, TapFn>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    TapFn: Fn(InputOk) -> OutputOk + Send + Sync,
{
    type OutputOk = OutputOk;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async { self.apply(input_ok) }
    }
}
