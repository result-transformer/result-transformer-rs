//! Flow utilities for transforming an entire [`Result`] asynchronously.
//!
//! These APIs are implemented with `async fn` and keep overhead low. For the
//! best performance, prefer implementing `AsyncResultTransformer` directly
//! instead of chaining flows.

use std::marker::PhantomData;

/// A transformation working on `Result` values in an asynchronous context.
pub trait AsyncResultFlow<InputOk, InputErr> {
    /// Success type produced by the flow.
    type OutputOk;
    /// Error type produced by the flow.
    type OutputErr;

    /// Apply the transformation to the given [`Result`].
    fn apply_result_async<'a>(
        &'a self,
        input: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a;

    /// Chain another [`AsyncResultFlow`] after this one.
    fn then_async_result<NextFlow>(
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

    fn apply_result_async<'a>(
        &'a self,
        input_result: Result<InputOk, InputErr>,
    ) -> impl Future<Output = Result<Self::OutputOk, Self::OutputErr>> + Send + 'a {
        async {
            let intermediate = self.head.apply_result_async(input_result).await;
            self.next.apply_result_async(intermediate).await
        }
    }
}
