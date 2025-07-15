use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrMapStep;

impl<InputErr, OutputErr, MapperFn> ErrFlow<InputErr> for ErrMapStep<InputErr, OutputErr, MapperFn>
where
    MapperFn: Fn(InputErr) -> OutputErr,
{
    type OutputErr = OutputErr;

    /// Implementation of [`ErrFlow::apply_err`].
    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        self.apply(input_err)
    }
}
