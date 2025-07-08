use std::marker::PhantomData;

/// Step that inspects the entire `Result` without modifying it.
#[derive(Debug, Clone, Copy)]
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

    pub(crate) fn apply(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<InputOk, InputErr> {
        (self.inspector)(&input_result);
        input_result
    }
}
