use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultInspectStep;

impl<OkType, ErrType, InspectorFn> ResultFlow<OkType, ErrType>
    for ResultInspectStep<OkType, ErrType, InspectorFn>
where
    InspectorFn: Fn(&Result<OkType, ErrType>),
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    /// Implementation of [`ResultFlow::apply_result`].
    ///
    /// Passes the `Result` to the inspector and returns the original value.
    fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        self.apply(input_result)
    }
}
