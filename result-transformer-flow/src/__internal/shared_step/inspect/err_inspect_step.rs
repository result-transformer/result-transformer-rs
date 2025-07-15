use std::marker::PhantomData;

/// Step that passes the error to an inspector and returns it unchanged.
#[derive(Debug, Clone, Copy)]
pub struct ErrInspectStep<ErrType, InspectorFn>
where
    InspectorFn: Fn(&ErrType),
{
    inspector: InspectorFn,
    _phantom: PhantomData<ErrType>,
}

impl<ErrType, InspectorFn> ErrInspectStep<ErrType, InspectorFn>
where
    InspectorFn: Fn(&ErrType),
{
    /// Creates a new [`ErrInspectStep`].
    ///
    /// * `inspector` - function that receives a reference to the error value
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_err: ErrType) -> ErrType {
        (self.inspector)(&input_err);
        input_err
    }
}
