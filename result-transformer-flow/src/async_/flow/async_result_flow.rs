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
    fn then_async_result<Next>(
        self,
        next: Next,
    ) -> AsyncResultFlowChain<InputOk, InputErr, Self, Next>
    where
        Self: Sized,
        Next: AsyncResultFlow<Self::OutputOk, Self::OutputErr>,
    {
        AsyncResultFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// Composition of two [`AsyncResultFlow`] implementations.
#[derive(Debug, Copy, Clone)]
pub struct AsyncResultFlowChain<InputOk, InputErr, Head, Next>
where
    Head: AsyncResultFlow<InputOk, InputErr>,
    Next: AsyncResultFlow<Head::OutputOk, Head::OutputErr>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<InputOk, InputErr, Head, Next> AsyncResultFlowChain<InputOk, InputErr, Head, Next>
where
    Head: AsyncResultFlow<InputOk, InputErr>,
    Next: AsyncResultFlow<Head::OutputOk, Head::OutputErr>,
{
    /// Creates a new [`AsyncResultFlowChain`].
    pub fn new(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }

    /// Creates a new [`AsyncResultFlowChain`] usable in const contexts.
    pub const fn new_const(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }
}

impl<InputOk, InputErr, Head, Next> AsyncResultFlow<InputOk, InputErr>
    for AsyncResultFlowChain<InputOk, InputErr, Head, Next>
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
