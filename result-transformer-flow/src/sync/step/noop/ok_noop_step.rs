use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

/// Step that returns the input value unchanged.
pub struct OkNoopStep<OkType> {
    _phantom: PhantomData<OkType>,
}

impl<OkType> OkNoopStep<OkType> {
    /// Creates a new [`OkNoopStep`].
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<OkType> OkFlow<OkType> for OkNoopStep<OkType> {
    type OutputOk = OkType;

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        input_ok
    }
}
