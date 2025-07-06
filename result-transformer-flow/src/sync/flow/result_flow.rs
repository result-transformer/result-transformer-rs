use std::marker::PhantomData;

/// A trait representing a transformation from `Result<InputOk, InputErr>` to
/// `Result<OutputOk, OutputErr>`.
///
/// Implementations may transform either the success or error value or both.
pub trait ResultFlow<InputOk, InputErr> {
    type OutputOk;
    type OutputErr;

    /// Applies the transformation to a `Result` value.
    ///
    /// # Parameters
    /// * `input` - The original result to transform.
    ///
    /// # Returns
    /// A new `Result` with the mapped success and error types.
    fn apply_result(
        &self,
        input: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr>;

    /// Chains this flow with another `ResultFlow`, passing the result of this
    /// flow as input to the next.
    fn then_result<NextFlow>(
        self,
        next: NextFlow,
    ) -> ResultFlowChain<Self, NextFlow, InputOk, InputErr>
    where
        Self: Sized,
        NextFlow: ResultFlow<Self::OutputOk, Self::OutputErr>,
    {
        ResultFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// A chained composition of two [`ResultFlow`]s.
/// The output of `head` becomes the input of `next`.
pub struct ResultFlowChain<Head, Next, InputOk, InputErr>
where
    Head: ResultFlow<InputOk, InputErr>,
    Next: ResultFlow<Head::OutputOk, Head::OutputErr>,
{
    /// The first flow in the chain.
    head: Head,
    /// The flow executed after `head`.
    next: Next,
    /// Marker to retain the `InputOk` and `InputErr` type parameters.
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<Head, Next, InputOk, InputErr> ResultFlow<InputOk, InputErr>
    for ResultFlowChain<Head, Next, InputOk, InputErr>
where
    Head: ResultFlow<InputOk, InputErr>,
    Next: ResultFlow<Head::OutputOk, Head::OutputErr>,
{
    type OutputOk = Next::OutputOk;
    type OutputErr = Next::OutputErr;

    fn apply_result(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        let intermediate = self.head.apply_result(input_result);
        self.next.apply_result(intermediate)
    }
}
