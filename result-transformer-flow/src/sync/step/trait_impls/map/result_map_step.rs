use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultMapStep;

impl<InputOk, InputErr, OutputOk, OutputErr, MapperFn> ResultFlow<InputOk, InputErr>
    for ResultMapStep<InputOk, InputErr, OutputOk, OutputErr, MapperFn>
where
    MapperFn: Fn(Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr>,
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
