use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultInspectStep;

impl<InspectorFn, InputOk, InputErr> ResultFlow<InputOk, InputErr>
    for ResultInspectStep<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>),
{
    type OutputOk = InputOk;
    type OutputErr = InputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    ///
    /// Passes the `Result` to the inspector and returns the original value.
    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
