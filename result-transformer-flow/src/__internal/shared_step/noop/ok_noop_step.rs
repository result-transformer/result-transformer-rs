use std::marker::PhantomData;

/// Step that returns the input value unchanged.
#[derive(Debug, Clone, Copy)]
pub struct OkNoopStep<OkType> {
    _phantom: PhantomData<OkType>,
}

impl<OkType> OkNoopStep<OkType> {
    /// Creates a new [`OkNoopStep`].
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_ok: OkType) -> OkType {
        input_ok
    }
}
