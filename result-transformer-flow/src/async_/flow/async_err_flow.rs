use result_transformer_dependencies::*;
use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait AsyncErrFlow<InputErr> {
    type OutputErr;

    async fn apply_err(&self, input: InputErr) -> Self::OutputErr;

    fn then_err<NextFlow>(self, next: NextFlow) -> AsyncErrFlowChain<Self, NextFlow, InputErr>
    where
        Self: Sized,
        NextFlow: AsyncErrFlow<Self::OutputErr>,
    {
        AsyncErrFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

pub struct AsyncErrFlowChain<Head, Next, InputErr>
where
    Head: AsyncErrFlow<InputErr>,
    Next: AsyncErrFlow<Head::OutputErr>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<InputErr>,
}

#[async_trait::async_trait]
impl<Head, Next, InputErr> AsyncErrFlow<InputErr> for AsyncErrFlowChain<Head, Next, InputErr>
where
    Head: AsyncErrFlow<InputErr> + Send + Sync,
    Next: AsyncErrFlow<Head::OutputErr> + Send + Sync,
    InputErr: Send + Sync,
{
    type OutputErr = Next::OutputErr;

    async fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        let intermediate = self.head.apply_err(input_err).await;
        self.next.apply_err(intermediate).await
    }
}
