use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

pub struct ResultNoopStep<OkType, ErrType> {
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultNoopStep<OkType, ErrType> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<OkType, ErrType> ResultFlow<OkType, ErrType> for ResultNoopStep<OkType, ErrType> {
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        input_result
    }
}
