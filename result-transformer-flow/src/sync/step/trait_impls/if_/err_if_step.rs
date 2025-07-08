use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrIfStep;

impl<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow> ErrFlow<InputErr>
    for ErrIfStep<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&InputErr) -> bool,
    ThenFlow: ErrFlow<InputErr, OutputErr = OutputErr>,
    ElseFlow: ErrFlow<InputErr, OutputErr = OutputErr>,
{
    type OutputErr = OutputErr;

    /// Implementation of [`ErrFlow::apply_err`].
    /// Evaluates the condition and passes the error to the selected flow.
    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        self.apply(input_err)
    }
}
