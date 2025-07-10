use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that inspects the entire `Result` without modifying it.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectStepAsync<InspectorFn, OkType, ErrType>
where
    InspectorFn: Fn(&Result<OkType, ErrType>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
        + Send
        + Sync,
{
    inspector: InspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<InspectorFn, OkType, ErrType> ResultInspectStepAsync<InspectorFn, OkType, ErrType>
where
    InspectorFn: Fn(&Result<OkType, ErrType>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
        + Send
        + Sync,
{
    /// Creates a new [`ResultInspectStepAsync`].
    ///
    /// * `inspector` - function receiving a reference to the `Result`
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

impl<InspectorFn, OkType, ErrType> AsyncResultFlow<OkType, ErrType>
    for ResultInspectStepAsync<InspectorFn, OkType, ErrType>
where
    InspectorFn: Fn(&Result<OkType, ErrType>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
        + Send
        + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<OkType, ErrType>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            (self.inspector)(&input_result).await;
            input_result
        }
    }
}
