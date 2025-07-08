use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultLogBothStep;

impl<OkType, ErrType> ResultFlow<OkType, ErrType> for ResultLogBothStep<OkType, ErrType> {
    type OutputOk = OkType;
    type OutputErr = ErrType;

    /// Implementation of [`ResultFlow::apply_result`].
    /// Logs the `Result` and then returns the original value.
    fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
