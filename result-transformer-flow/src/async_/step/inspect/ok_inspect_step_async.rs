use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncOkFlow;

/// Step that passes the success value to an inspector and returns it.
#[derive(Debug, Clone, Copy)]
pub struct OkInspectStepAsync<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
{
    inspector: InspectorFn,
    _phantom: PhantomData<OkType>,
}

impl<InspectorFn, OkType> OkInspectStepAsync<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
{
    /// Creates a new [`OkInspectStepAsync`].
    ///
    /// * `inspector` - function receiving a reference to the success value
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<InspectorFn, OkType> AsyncOkFlow<OkType> for OkInspectStepAsync<InspectorFn, OkType>
where
    InspectorFn: Fn(&OkType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    OkType: Send + Sync,
{
    type OutputOk = OkType;

    async fn apply_ok(&self, input_ok: OkType) -> Self::OutputOk {
        (self.inspector)(&input_ok).await;
        input_ok
    }
}
