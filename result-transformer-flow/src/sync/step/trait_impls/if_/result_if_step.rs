use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultIfStep;

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
