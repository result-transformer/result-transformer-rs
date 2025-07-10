use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncErrFlow;

/// Step that passes the error to an inspector and returns it unchanged.
#[derive(Debug, Clone, Copy)]
pub struct ErrInspectStepAsync<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    ErrType: Send + Sync,
{
    inspector: InspectorFn,
    _phantom: PhantomData<ErrType>,
}

impl<InspectorFn, ErrType> ErrInspectStepAsync<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    ErrType: Send + Sync,
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

impl<InspectorFn, ErrType> AsyncErrFlow<ErrType> for ErrInspectStepAsync<InspectorFn, ErrType>
where
    InspectorFn: Fn(&ErrType) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync,
    ErrType: Send + Sync,
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
