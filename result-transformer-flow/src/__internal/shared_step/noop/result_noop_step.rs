use std::marker::PhantomData;

/// Step that forwards the given `Result` without modification.
#[derive(Debug, Clone, Copy)]
pub struct ResultNoopStep<OkType, ErrType> {
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultNoopStep<OkType, ErrType> {
    /// Creates a new [`ResultNoopStep`].
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        input_result
    }
}
