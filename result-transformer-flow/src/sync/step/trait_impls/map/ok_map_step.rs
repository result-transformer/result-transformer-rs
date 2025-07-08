use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkMapStep;

impl<MapperFn, InputOk, OutputOk> OkFlow<InputOk> for OkMapStep<MapperFn, InputOk, OutputOk>
where
    MapperFn: Fn(InputOk) -> OutputOk,
{
    type OutputOk = OutputOk;

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
