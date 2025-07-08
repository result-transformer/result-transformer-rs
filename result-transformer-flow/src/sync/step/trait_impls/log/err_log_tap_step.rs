use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrLogTapStep;

impl<ErrType> ErrFlow<ErrType> for ErrLogTapStep<ErrType> {
    type OutputErr = ErrType;

    /// Implementation of [`ErrFlow::apply_err`].
    /// Logs the value and then returns the original error.
    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        self.apply(input_err)
    }
}
