use result_transformer_dependencies::*;

use std::{marker::PhantomData, pin::Pin};

use crate::async_::AsyncResultFlow;

/// Step that inspects the entire `Result` without modifying it.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectStepAsync<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
        + Send
        + Sync,
{
    inspector: InspectorFn,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InspectorFn, InputOk, InputErr> ResultInspectStepAsync<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
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

#[async_trait::async_trait]
impl<InspectorFn, InputOk, InputErr> AsyncResultFlow<InputOk, InputErr>
    for ResultInspectStepAsync<InspectorFn, InputOk, InputErr>
where
    InspectorFn: Fn(&Result<InputOk, InputErr>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>
        + Send
        + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
{
    type OutputOk = InputOk;
    type OutputErr = InputErr;

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        (self.inspector)(&input_result).await;
        input_result
    }
}
