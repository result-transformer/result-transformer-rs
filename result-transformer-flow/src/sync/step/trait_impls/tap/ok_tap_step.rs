use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkTapStep;

impl<TapFn, InputOk, OutputOk> OkFlow<InputOk> for OkTapStep<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    type OutputOk = OutputOk;

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
