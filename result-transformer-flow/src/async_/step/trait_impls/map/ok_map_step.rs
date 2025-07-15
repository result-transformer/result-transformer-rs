//! Async implementation of [`OkMapStep`].

use crate::__internal::shared_step::OkMapStep;
use crate::async_::AsyncOkFlow;

impl<InputOk, OutputOk, MapperFn> AsyncOkFlow<InputOk> for OkMapStep<InputOk, OutputOk, MapperFn>
where
    InputOk: Send + Sync,
    OutputOk: Send + Sync,
    MapperFn: Fn(InputOk) -> OutputOk + Send + Sync,
{
    type OutputOk = OutputOk;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async { self.apply(input_ok) }
    }
}
