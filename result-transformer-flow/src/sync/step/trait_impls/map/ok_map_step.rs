use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkMapStep;

impl<InputOk, OutputOk, MapperFn> OkFlow<InputOk> for OkMapStep<InputOk, OutputOk, MapperFn>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    type OutputOk = OutputOk;

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
