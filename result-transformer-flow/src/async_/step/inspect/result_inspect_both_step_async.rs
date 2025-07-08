use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that inspects both the success and error values without modifying them.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectBothStepAsync<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
{
    ok_inspector: OkInspectorFn,
    err_inspector: ErrInspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
    ResultInspectBothStepAsync<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
{
    /// Creates a new [`ResultInspectBothStepAsync`].
    ///
    /// * `ok_fn` - inspector for the success value
    /// * `err_fn` - inspector for the error value
    pub fn new(ok_fn: OkInspectorFn, err_fn: ErrInspectorFn) -> Self {
        Self {
            ok_inspector: ok_fn,
            err_inspector: err_fn,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType> AsyncResultFlow<OkType, ErrType>
    for ResultInspectBothStepAsync<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    OkType: Send + Sync,
    ErrType: Send + Sync,
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    async fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        match &input_result {
            Ok(ok) => (self.ok_inspector)(ok).await,
            Err(err) => (self.err_inspector)(err).await,
        }
        input_result
    }
}
