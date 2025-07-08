use crate::sync::flow::OkFlow;

use crate::__internal::shared_step::OkLogTapStep;

impl<OkType> OkFlow<OkType> for OkLogTapStep<OkType> {
    type OutputOk = OkType;

    /// Implementation of [`OkFlow::apply_ok`].
    /// Logs the value and returns the original success.
    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        self.apply(input_ok)
    }
}
