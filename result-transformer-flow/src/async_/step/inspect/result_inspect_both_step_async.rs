use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncResultFlow;

/// Step that inspects both the success and error values without modifying them.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectBothStepAsync<
    OkType,
    ErrType,
    OkInspectorFn,
    ErrInspectorFn,
    OkInspectorFut,
    ErrInspectorFut,
> where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    OkInspectorFn: Fn(&OkType) -> OkInspectorFut + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> ErrInspectorFut + Send + Sync,
    OkInspectorFut: Future<Output = ()> + Send,
    ErrInspectorFut: Future<Output = ()> + Send,
{
    ok_inspector: OkInspectorFn,
    err_inspector: ErrInspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType, OkInspectorFn, ErrInspectorFn, OkInspectorFut, ErrInspectorFut>
    ResultInspectBothStepAsync<
        OkType,
        ErrType,
        OkInspectorFn,
        ErrInspectorFn,
        OkInspectorFut,
        ErrInspectorFut,
    >
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    OkInspectorFn: Fn(&OkType) -> OkInspectorFut + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> ErrInspectorFut + Send + Sync,
    OkInspectorFut: Future<Output = ()> + Send,
    ErrInspectorFut: Future<Output = ()> + Send,
{
    /// Creates a new [`ResultInspectBothStepAsync`].
    ///
    /// * `ok_fn` - inspector for the success value
    /// * `err_fn` - inspector for the error value
    pub const fn new(ok_fn: OkInspectorFn, err_fn: ErrInspectorFn) -> Self {
        Self {
            ok_inspector: ok_fn,
            err_inspector: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkType, ErrType, OkInspectorFn, ErrInspectorFn, OkInspectorFut, ErrInspectorFut>
    AsyncResultFlow<OkType, ErrType>
    for ResultInspectBothStepAsync<
        OkType,
        ErrType,
        OkInspectorFn,
        ErrInspectorFn,
        OkInspectorFut,
        ErrInspectorFut,
    >
where
    OkType: Send + Sync,
    ErrType: Send + Sync,
    OkInspectorFn: Fn(&OkType) -> OkInspectorFut + Send + Sync,
    ErrInspectorFn: Fn(&ErrType) -> ErrInspectorFut + Send + Sync,
    OkInspectorFut: Future<Output = ()> + Send,
    ErrInspectorFut: Future<Output = ()> + Send,
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<OkType, ErrType>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            match &input_result {
                Ok(ok) => (self.ok_inspector)(ok).await,
                Err(err) => (self.err_inspector)(err).await,
            }
            input_result
        }
    }
}
