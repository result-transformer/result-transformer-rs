use result_transformer_dependencies::*;
use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait AsyncOkFlow<InputOk> {
    type OutputOk;

    async fn apply_ok(&self, input: InputOk) -> Self::OutputOk;

    fn then_ok<NextFlow>(self, next: NextFlow) -> AsyncOkFlowChain<Self, NextFlow, InputOk>
    where
        Self: Sized,
        NextFlow: AsyncOkFlow<Self::OutputOk>,
    {
        AsyncOkFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

pub struct AsyncOkFlowChain<Head, Next, InputOk>
where
    Head: AsyncOkFlow<InputOk>,
    Next: AsyncOkFlow<Head::OutputOk>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<InputOk>,
}

#[async_trait::async_trait]
impl<Head, Next, InputOk> AsyncOkFlow<InputOk> for AsyncOkFlowChain<Head, Next, InputOk>
where
    Head: AsyncOkFlow<InputOk> + Send + Sync,
    Next: AsyncOkFlow<Head::OutputOk> + Send + Sync,
    InputOk: Send + Sync,
{
    type OutputOk = Next::OutputOk;

    async fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        let intermediate = self.head.apply_ok(input_ok).await;
        self.next.apply_ok(intermediate).await
    }
}
