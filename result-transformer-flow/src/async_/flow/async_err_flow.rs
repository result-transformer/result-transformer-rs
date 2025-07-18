//! Flow utilities for transforming only the `Err` variant asynchronously.
//!
//! These helpers use standard `async fn` and are lightweight. When possible,
//! define your own `AsyncErrTransformer` instead of composing these flows.

use std::marker::PhantomData;

/// A transformation that only manipulates the error value of an asynchronous computation.
pub trait AsyncErrFlow<InputErr> {
    /// Resulting error type after the flow is applied.
    type OutputErr;

    /// Apply the transformation to the given error value.
    fn apply_err_async<'a>(
        &'a self,
        input: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a;

    /// Chain another [`AsyncErrFlow`] to process the output of this one.
    fn then_async_err<Next>(self, next: Next) -> AsyncErrFlowChain<InputErr, Self, Next>
    where
        Self: Sized,
        Next: AsyncErrFlow<Self::OutputErr>,
    {
        AsyncErrFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// Composition of two [`AsyncErrFlow`] implementations.
#[derive(Debug, Copy, Clone)]
pub struct AsyncErrFlowChain<InputErr, Head, Next>
where
    Head: AsyncErrFlow<InputErr>,
    Next: AsyncErrFlow<Head::OutputErr>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<InputErr>,
}

impl<InputErr, Head, Next> AsyncErrFlowChain<InputErr, Head, Next>
where
    Head: AsyncErrFlow<InputErr>,
    Next: AsyncErrFlow<Head::OutputErr>,
{
    /// Creates a new [`AsyncErrFlowChain`].
    pub fn new(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }

    /// Creates a new [`AsyncErrFlowChain`] usable in const contexts.
    pub const fn new_const(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }
}

impl<InputErr, Head, Next> AsyncErrFlow<InputErr> for AsyncErrFlowChain<InputErr, Head, Next>
where
    Head: AsyncErrFlow<InputErr> + Send + Sync,
    Next: AsyncErrFlow<Head::OutputErr> + Send + Sync,
    InputErr: Send + Sync,
{
    type OutputErr = Next::OutputErr;

    fn apply_err_async<'a>(
        &'a self,
        input_err: InputErr,
    ) -> impl Future<Output = Self::OutputErr> + Send + 'a {
        async {
            let intermediate = self.head.apply_err_async(input_err).await;
            self.next.apply_err_async(intermediate).await
        }
    }
}
