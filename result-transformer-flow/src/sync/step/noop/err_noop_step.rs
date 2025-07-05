use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

pub struct ErrNoopStep<ErrType> {
    _phantom: PhantomData<ErrType>,
}

impl<ErrType> ErrNoopStep<ErrType> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<ErrType> ErrFlow<ErrType> for ErrNoopStep<ErrType> {
    type OutputErr = ErrType;

    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        input_err
    }
}
