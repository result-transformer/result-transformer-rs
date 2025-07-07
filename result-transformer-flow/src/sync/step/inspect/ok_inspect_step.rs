use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

/// Step that passes the success value to an inspector and returns it.
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
    /// Creates a new [`OkInspectStep`].
    ///
    /// * `inspector` - function receiving a reference to the success value
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

    /// Implementation of [`OkFlow::apply_ok`].
    /// Passes the success value to the inspector and returns it.
    fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        (self.inspector)(&input_ok);
        input_ok
    }
}
