use std::marker::PhantomData;

/// Step that returns the error value unchanged.
#[derive(Debug, Clone, Copy)]
pub struct ErrNoopStep<ErrType> {
    _phantom: PhantomData<ErrType>,
}

impl<ErrType> ErrNoopStep<ErrType> {
    /// Creates a new [`ErrNoopStep`].
    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_err: ErrType) -> ErrType {
        input_err
    }
}
