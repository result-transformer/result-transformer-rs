use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Conditionally applies one of two [`ResultFlow`]s.
pub struct ResultIfStep<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow> {
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    ResultIfStep<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&Result<InputOk, InputErr>) -> bool,
    ThenFlow: ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>,
    ElseFlow: ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>,
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

impl<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    ResultFlow<InputOk, InputErr>
    for ResultIfStep<InputOk, InputErr, OutputOk, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&Result<InputOk, InputErr>) -> bool,
    ThenFlow: ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>,
    ElseFlow: ResultFlow<InputOk, InputErr, OutputOk = OutputOk, OutputErr = OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        if (self.condition)(&input_result) {
            self.then_flow.apply_result(input_result)
        } else {
            self.else_flow.apply_result(input_result)
        }
    }
}
