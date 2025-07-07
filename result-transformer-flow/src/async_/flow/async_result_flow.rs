//! Flow utilities for transforming an entire [`Result`] asynchronously.
//!
//! These APIs leverage [`async-trait`](https://docs.rs/async-trait) which
//! introduces a small amount of overhead. For the best performance, prefer
//! implementing `AsyncResultTransformer` directly instead of chaining flows.

use result_transformer_dependencies::*;
use std::marker::PhantomData;

/// A transformation working on `Result` values in an asynchronous context.
#[async_trait::async_trait]
pub trait AsyncResultFlow<InputOk, InputErr> {
    /// Success type produced by the flow.
    type OutputOk;
    /// Error type produced by the flow.
    type OutputErr;

    /// Apply the transformation to the given [`Result`].
    async fn apply_result(
        &self,
        input: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr>;

    /// Chain another [`AsyncResultFlow`] after this one.
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

/// Composition of two [`AsyncResultFlow`] implementations.
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
