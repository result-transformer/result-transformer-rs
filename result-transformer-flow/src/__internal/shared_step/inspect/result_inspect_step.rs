use std::marker::PhantomData;

/// Step that inspects the entire `Result` without modifying it.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectStep<OkType, ErrType, InspectorFn>
where
    InspectorFn: Fn(&Result<OkType, ErrType>),
{
    inspector: InspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType, InspectorFn> ResultInspectStep<OkType, ErrType, InspectorFn>
where
    InspectorFn: Fn(&Result<OkType, ErrType>),
{
    /// Creates a new [`ResultInspectStep`].
    ///
    /// * `inspector` - function receiving a reference to the `Result`
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        (self.inspector)(&input_result);
        input_result
    }
}
