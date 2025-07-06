use result_transformer_dependencies::*;
use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait AsyncResultFlow<InputOk, InputErr> {
    type OutputOk;
    type OutputErr;

    async fn apply_result(
        &self,
        input: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr>;

    fn then_result<NextFlow>(
        self,
        next: NextFlow,
    ) -> AsyncResultFlowChain<Self, NextFlow, InputOk, InputErr>
    where
        Self: Sized,
        NextFlow: AsyncResultFlow<Self::OutputOk, Self::OutputErr>,
    {
        AsyncResultFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

pub struct AsyncResultFlowChain<Head, Next, InputOk, InputErr>
where
    Head: AsyncResultFlow<InputOk, InputErr>,
    Next: AsyncResultFlow<Head::OutputOk, Head::OutputErr>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

#[async_trait::async_trait]
impl<Head, Next, InputOk, InputErr> AsyncResultFlow<InputOk, InputErr>
    for AsyncResultFlowChain<Head, Next, InputOk, InputErr>
where
    Head: AsyncResultFlow<InputOk, InputErr> + Send + Sync,
    Next: AsyncResultFlow<Head::OutputOk, Head::OutputErr> + Send + Sync,
    InputOk: Send + Sync,
    InputErr: Send + Sync,
{
    type OutputOk = Next::OutputOk;
    type OutputErr = Next::OutputErr;

    async fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        let intermediate = self.head.apply_result(input_result).await;
        self.next.apply_result(intermediate).await
    }
}
