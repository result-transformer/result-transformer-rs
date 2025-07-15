use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

/// Step that executes one of two [`OkFlow`] branches depending on a condition.
#[derive(Debug, Clone, Copy)]
pub struct OkIfStep<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow> {
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
    OkIfStep<InputOk, OutputOk, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&InputOk) -> bool,
    ThenFlow: OkFlow<InputOk, OutputOk = OutputOk>,
    ElseFlow: OkFlow<InputOk, OutputOk = OutputOk>,
{
    /// Creates a new [`OkIfStep`].
    ///
    /// * `condition` - predicate that evaluates the successful value
    /// * `then_flow` - flow run when the predicate returns `true`
    /// * `else_flow` - flow run when the predicate returns `false`
    pub const fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_ok: InputOk) -> OutputOk {
        if (self.condition)(&input_ok) {
            self.then_flow.apply_ok(input_ok)
        } else {
            self.else_flow.apply_ok(input_ok)
        }
    }
}

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
