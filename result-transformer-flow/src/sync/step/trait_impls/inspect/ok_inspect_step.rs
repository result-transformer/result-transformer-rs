use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkInspectStep;

impl<OkType, InspectorFn> OkFlow<OkType> for OkInspectStep<OkType, InspectorFn>
where
    InspectorFn: Fn(&OkType),
{
    type OutputOk = OkType;

    /// Implementation of [`OkFlow::apply_ok`].
    /// Passes the success value to the inspector and returns it.
    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
