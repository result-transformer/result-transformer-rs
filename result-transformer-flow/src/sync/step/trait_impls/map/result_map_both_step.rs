use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultMapBothStep;

impl<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn> ResultFlow<InputOk, InputErr>
    for ResultMapBothStep<InputOk, InputErr, OutputOk, OutputErr, OkMapperFn, ErrMapperFn>
where
    OkMapperFn: Fn(InputOk) -> OutputOk,
    ErrMapperFn: Fn(InputErr) -> OutputErr,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(&self, input_result: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        self.apply(input_result)
    }
}
