use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncOkFlow;

/// Step that passes the success value to an inspector and returns it.
#[derive(Debug, Clone, Copy)]
pub struct OkInspectStepAsync<OkType, InspectorFn, InspectorFut>
where
    OkType: Send + Sync,
    InspectorFn: Fn(&OkType) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    inspector: InspectorFn,
    _phantom: PhantomData<OkType>,
}

impl<OkType, InspectorFn, InspectorFut> OkInspectStepAsync<OkType, InspectorFn, InspectorFut>
where
    OkType: Send + Sync,
    InspectorFn: Fn(&OkType) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    /// Creates a new [`OkInspectStepAsync`].
    ///
    /// * `inspector` - function receiving a reference to the success value
    pub const fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

impl<OkType, InspectorFn, InspectorFut> AsyncOkFlow<OkType>
    for OkInspectStepAsync<OkType, InspectorFn, InspectorFut>
where
    OkType: Send + Sync,
    InspectorFn: Fn(&OkType) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    type OutputOk = OkType;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: OkType,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async {
            (self.inspector)(&input_ok).await;
            input_ok
        }
    }
}
