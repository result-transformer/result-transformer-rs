use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

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
    pub fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
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

    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        if (self.condition)(&input_ok) {
            self.then_flow.apply_ok(input_ok)
        } else {
            self.else_flow.apply_ok(input_ok)
        }
    }
}
