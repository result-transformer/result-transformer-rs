use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultTapStep;

impl<InputOk, InputErr, OutputOk, OutputErr, TapFn> ResultFlow<InputOk, InputErr>
    for ResultTapStep<InputOk, InputErr, OutputOk, OutputErr, TapFn>
where
    TapFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
