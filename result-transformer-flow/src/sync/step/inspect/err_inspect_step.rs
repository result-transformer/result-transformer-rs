use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Step that passes the error to an inspector and returns it unchanged.
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
    /// Creates a new [`ErrInspectStep`].
    ///
    /// * `inspector` - function that receives a reference to the error value
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

    /// Implementation of [`ErrFlow::apply_err`].
    /// Passes the error to the inspector and then returns it.
    fn apply_err(&self, input_err: ErrType) -> Self::OutputErr {
        (self.inspector)(&input_err);
        input_err
    }
}
