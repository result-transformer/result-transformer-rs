use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

pub struct OkNoopStep<OkType> {
    _phantom: PhantomData<OkType>,
}

impl<OkType> OkNoopStep<OkType> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<OkType> OkFlow<OkType> for OkNoopStep<OkType> {
    type OutputOk = OkType;

    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        input_ok
    }
}
