//! Flow utilities for transforming only the `Err` variant asynchronously.
//!
//! These helpers are built on top of [`async-trait`](https://docs.rs/async-trait)
//! and therefore carry a small runtime penalty. When possible, define your own
//! `AsyncErrTransformer` instead of composing these flows.

use result_transformer_dependencies::*;
use std::marker::PhantomData;

/// A transformation that only manipulates the error value of an asynchronous computation.
#[async_trait::async_trait]
pub trait AsyncErrFlow<InputErr> {
    /// Resulting error type after the flow is applied.
    type OutputErr;

    /// Apply the transformation to the given error value.
    async fn apply_err(&self, input: InputErr) -> Self::OutputErr;

    /// Chain another [`AsyncErrFlow`] to process the output of this one.
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

/// Composition of two [`AsyncErrFlow`] implementations.
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
