use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Step that inspects the entire `Result` without modifying it.
pub struct ResultInspectStep<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>),
{
    inspector: InspectorFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InspectorFn, InputOk, InputErr> ResultInspectStep<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>),
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
}

impl<InspectorFn, InputOk, InputErr> ResultFlow<InputOk, InputErr>
    for ResultInspectStep<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>),
{
    type OutputOk = InputOk;
    type OutputErr = InputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    ///
    /// Passes the `Result` to the inspector and returns the original value.
    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        (self.inspector)(&input_result);
        input_result
    }
}
