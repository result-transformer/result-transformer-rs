use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrTapStep;

impl<InputErr, OutputErr, TapFn> ErrFlow<InputErr> for ErrTapStep<InputErr, OutputErr, TapFn>
where
    TapFn: Fn(InputErr) -> OutputErr,
{
    type OutputErr = OutputErr;

    /// Implementation of [`ErrFlow::apply_err`].
    fn apply_err(&self, input_ok: InputErr) -> Self::OutputErr {
        self.apply(input_ok)
    }
}
