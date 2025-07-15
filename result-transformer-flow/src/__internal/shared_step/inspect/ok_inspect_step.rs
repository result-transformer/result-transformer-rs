use std::marker::PhantomData;

/// Step that passes the success value to an inspector and returns it.
#[derive(Debug, Clone, Copy)]
pub struct OkInspectStep<OkType, InspectorFn>
where
    InspectorFn: Fn(&OkType),
{
    inspector: InspectorFn,
    _phantom: PhantomData<OkType>,
}

impl<InspectorFn, OkType> OkInspectStep<OkType, InspectorFn>
where
    InspectorFn: Fn(&OkType),
{
    /// Creates a new [`OkInspectStep`].
    ///
    /// * `inspector` - function receiving a reference to the success value
    pub const fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }


    pub(crate) fn apply(&self, input_err: OkType) -> OkType {
        (self.inspector)(&input_err);
        input_err
    }
}
