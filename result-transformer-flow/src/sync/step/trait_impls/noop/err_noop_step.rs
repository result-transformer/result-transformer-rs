use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrNoopStep;

impl<ErrType> ErrFlow<ErrType> for ErrNoopStep<ErrType> {
    type OutputErr = ErrType;

    /// Implementation of [`ErrFlow::apply_err`].
    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        self.apply(input_err)
    }
}
