use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkNoopStep;

impl<OkType> OkFlow<OkType> for OkNoopStep<OkType> {
    type OutputOk = OkType;

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
