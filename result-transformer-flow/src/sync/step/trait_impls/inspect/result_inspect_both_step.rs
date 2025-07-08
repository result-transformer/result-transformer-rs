use crate::sync::flow::ResultFlow;

use crate::__internal::shared_step::ResultInspectBothStep;

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType> ResultFlow<OkType, ErrType>
    for ResultInspectBothStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType),
    ErrInspectorFn: Fn(&ErrType),
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    /// Implementation of [`ResultFlow::apply_result`].
    ///
    /// Inspects the `Result` contents and then returns the original value.
    fn apply_result(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        self.apply(input_result)
    }
}
