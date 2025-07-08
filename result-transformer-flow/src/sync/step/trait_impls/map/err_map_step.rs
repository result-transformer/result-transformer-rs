use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrMapStep;

impl<MapperFn, InputErr, OutputErr> ErrFlow<InputErr> for ErrMapStep<MapperFn, InputErr, OutputErr>
where
    MapperFn: Fn(InputErr) -> OutputErr,
{
    type OutputErr = OutputErr;

    /// Implementation of [`ErrFlow::apply_err`].
    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        self.apply(input_err)
    }
}
