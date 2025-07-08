use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkIfStep;

impl<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow> OkFlow<InputOk>
    for OkIfStep<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&InputOk) -> bool,
    ThenFlow: OkFlow<InputOk, OutputOk = OutputOk>,
    ElseFlow: OkFlow<InputOk, OutputOk = OutputOk>,
{
    type OutputOk = OutputOk;

    /// Implementation of [`OkFlow::apply_ok`].
    /// Evaluates the condition and forwards the value to the chosen flow.
    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
