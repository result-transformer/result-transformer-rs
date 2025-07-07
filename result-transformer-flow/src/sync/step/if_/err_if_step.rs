use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Executes one of two [`ErrFlow`]s depending on the condition.
///
/// `InputErr` denotes the input error type, and `OutputErr` the output error type.
pub struct ErrIfStep<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow> {
    condition: ConditionFn,
    then_flow: ThenFlow,
    else_flow: ElseFlow,
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
    ErrIfStep<InputErr, OutputErr, ConditionFn, ThenFlow, ElseFlow>
where
    ConditionFn: Fn(&InputErr) -> bool,
    ThenFlow: ErrFlow<InputErr, OutputErr = OutputErr>,
    ElseFlow: ErrFlow<InputErr, OutputErr = OutputErr>,
{
    /// Creates a new [`ErrIfStep`].
    ///
    /// * `condition` - predicate that evaluates the error value
    /// * `then_flow` - flow executed when the predicate is `true`
    /// * `else_flow` - flow executed when the predicate is `false`
    pub fn new(condition: ConditionFn, then_flow: ThenFlow, else_flow: ElseFlow) -> Self {
        Self {
            condition,
            then_flow,
            else_flow,
            _phantom: PhantomData,
        }
    }
}

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
        if (self.condition)(&input_err) {
            self.then_flow.apply_err(input_err)
        } else {
            self.else_flow.apply_err(input_err)
        }
    }
}
