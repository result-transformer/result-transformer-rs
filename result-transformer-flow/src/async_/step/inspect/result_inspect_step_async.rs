use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that inspects the entire `Result` without modifying it.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectStepAsync<OkType, ErrType, InspectorFn, InspectorFut>
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    InspectorFn: Fn(&Result<OkType, ErrType>) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    inspector: InspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType, InspectorFn, InspectorFut>
    ResultInspectStepAsync<OkType, ErrType, InspectorFn, InspectorFut>
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    InspectorFn: Fn(&Result<OkType, ErrType>) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    /// Creates a new [`ResultInspectStepAsync`].
    ///
    /// * `inspector` - function receiving a reference to the `Result`
    pub const fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

impl<OkType, ErrType, InspectorFn, InspectorFut> AsyncResultFlow<OkType, ErrType>
    for ResultInspectStepAsync<OkType, ErrType, InspectorFn, InspectorFut>
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    InspectorFn: Fn(&Result<OkType, ErrType>) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
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
