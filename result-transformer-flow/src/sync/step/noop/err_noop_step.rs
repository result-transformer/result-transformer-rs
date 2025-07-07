use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Step that returns the error value unchanged.
pub struct ErrNoopStep<ErrType> {
    _phantom: PhantomData<ErrType>,
}

impl<ErrType> ErrNoopStep<ErrType> {
    /// Creates a new [`ErrNoopStep`].
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<ErrType> ErrFlow<ErrType> for ErrNoopStep<ErrType> {
    type OutputErr = ErrType;

    /// Implementation of [`ErrFlow::apply_err`].
    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        input_err
    }
}
