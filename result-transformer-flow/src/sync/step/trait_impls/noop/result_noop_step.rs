use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultNoopStep;

impl<OkType, ErrType> ResultFlow<OkType, ErrType> for ResultNoopStep<OkType, ErrType> {
    type OutputOk = OkType;
    type OutputErr = ErrType;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
