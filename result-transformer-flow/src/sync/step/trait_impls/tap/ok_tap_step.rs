use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkTapStep;

impl<InputOk, OutputOk, TapFn> OkFlow<InputOk> for OkTapStep<InputOk, OutputOk, TapFn>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    type OutputOk = OutputOk;

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
