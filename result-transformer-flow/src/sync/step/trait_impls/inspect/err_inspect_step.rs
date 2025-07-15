use crate::sync::flow::ErrFlow;

use crate::__internal::shared_step::ErrInspectStep;

impl<InspectorFn, ErrType> ErrFlow<ErrType> for ErrInspectStep<ErrType, InspectorFn>
where
    InspectorFn: Fn(&ErrType),
{
    type OutputErr = ErrType;

    /// Implementation of [`ErrFlow::apply_err`].
    /// Passes the error to the inspector and then returns it.
    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        self.apply(input_err)
    }
}
