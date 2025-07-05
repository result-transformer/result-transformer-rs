use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

pub struct ErrInspectStep<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType),
{
    inspector: InspectorFn,
    _phantom: PhantomData<ErrType>,
}

impl<InspectorFn, ErrType> ErrInspectStep<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType),
{
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

impl<InspectorFn, ErrType> ErrFlow<ErrType> for ErrInspectStep<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType),
{
    type OutputErr = ErrType;

    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        (self.inspector)(&input_err);
        input_err
    }
}
