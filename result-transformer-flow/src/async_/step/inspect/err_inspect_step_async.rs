use core::future::Future;
use std::marker::PhantomData;

use crate::async_::AsyncErrFlow;

/// Step that passes the error to an inspector and returns it unchanged.
#[derive(Debug, Clone, Copy)]
pub struct ErrInspectStepAsync<ErrType, InspectorFn, InspectorFut>
where
    ErrType: Send + Sync,
    InspectorFn: Fn(&ErrType) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    inspector: InspectorFn,
    _phantom: PhantomData<ErrType>,
}

impl<ErrType, InspectorFn, InspectorFut> ErrInspectStepAsync<ErrType, InspectorFn, InspectorFut>
where
    ErrType: Send + Sync,
    InspectorFn: Fn(&ErrType) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    /// Creates a new [`ErrInspectStepAsync`].
    ///
    /// * `inspector` - function that receives a reference to the error value
    pub fn new(inspector: InspectorFn) -> Self {
        Self {
            inspector,
            _phantom: PhantomData,
        }
    }
}

impl<ErrType, InspectorFn, InspectorFut> AsyncErrFlow<ErrType>
    for ErrInspectStepAsync<ErrType, InspectorFn, InspectorFut>
where
    ErrType: Send + Sync,
    InspectorFn: Fn(&ErrType) -> InspectorFut + Send + Sync,
    InspectorFut: Future<Output = ()> + Send,
{
    type OutputErr = ErrType;

    fn apply_err_async<'a>(
        &'a self,
        input_err: ErrType,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async {
            (self.inspector)(&input_err).await;
            input_err
        }
    }
}
