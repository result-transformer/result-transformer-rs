use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultMapBothBindStep;

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(&self, input_result: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        self.apply(input_result)
    }
}
