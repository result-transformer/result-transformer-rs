//! Flow utilities for transforming only the `Ok` variant asynchronously.
//!
//! These helpers are built using `async fn` and are lightweight. When possible,
//! implement an `AsyncOkTransformer` yourself instead of composing flows.

use std::marker::PhantomData;

/// A transformation that operates on the success value of an asynchronous computation.
pub trait AsyncOkFlow<InputOk> {
    /// Resulting success type after the flow is applied.
    type OutputOk;

    /// Apply the transformation to the provided value.
    fn apply_ok_async<'a>(
        &'a self,
        input: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a;

    /// Chain another [`AsyncOkFlow`] to be executed after this one.
    fn then_async_ok<NextFlow>(self, next: NextFlow) -> AsyncOkFlowChain<Self, NextFlow, InputOk>
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

/// Composition of two [`AsyncOkFlow`] implementations.
pub struct AsyncOkFlowChain<Head, Next, InputOk>
where
    Head: AsyncOkFlow<InputOk>,
    Next: AsyncOkFlow<Head::OutputOk>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<InputOk>,
}

impl<Head, Next, InputOk> AsyncOkFlow<InputOk> for AsyncOkFlowChain<Head, Next, InputOk>
where
    Head: AsyncOkFlow<InputOk> + Send + Sync,
    Next: AsyncOkFlow<Head::OutputOk> + Send + Sync,
    InputOk: Send + Sync,
{
    type OutputOk = Next::OutputOk;

    fn apply_ok_async<'a>(
        &'a self,
        input_ok: InputOk,
    ) -> impl Future<Output = Self::OutputOk> + Send + 'a {
        async {
            let intermediate = self.head.apply_ok_async(input_ok).await;
            self.next.apply_ok_async(intermediate).await
        }
    }
}
