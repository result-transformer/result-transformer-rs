use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

pub struct OkInspectStep<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType),
{
    inspector: InspectorFn,
    _phantom: PhantomData<OkType>,
}

impl<InspectorFn, OkType> OkInspectStep<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType),
{
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

impl<InspectorFn, OkType> OkFlow<OkType> for OkInspectStep<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType),
{
    type OutputOk = OkType;

    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        (self.inspector)(&input_ok);
        input_ok
    }
}
