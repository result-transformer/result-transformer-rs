use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Step that applies one of two [`ResultFlow`]s depending on a predicate.
#[derive(Debug, Clone, Copy)]
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
    /// Creates a new [`ResultIfStep`].
    ///
    /// * `condition` - function that inspects the entire `Result`
    /// * `then_flow` - flow executed when the predicate returns `true`
    /// * `else_flow` - flow executed when the predicate returns `false`
    pub fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<OutputOk, OutputErr> {
        if (self.condition)(&input_result) {
            self.then_flow.apply_result(input_result)
        } else {
            self.else_flow.apply_result(input_result)
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

    /// Implementation of [`ResultFlow::apply_result`].
    ///
    /// Delegates processing to `then_flow` or `else_flow` based on the predicate.
    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
