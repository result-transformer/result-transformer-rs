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
    fn then_result<Next>(
        self,
        next: Next,
    ) -> ResultFlowChain<Self, Next, InputOk, InputErr>
    where
        Self: Sized,
        Next: ResultFlow<Self::OutputOk, Self::OutputErr>,
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

// `Clone` implementation when both flows are cloneable
impl<Head, Next, InputOk, InputErr> Clone for ResultFlowChain<Head, Next, InputOk, InputErr>
where
    Head: ResultFlow<InputOk, InputErr> + Clone,
    Next: ResultFlow<Head::OutputOk, Head::OutputErr> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            next: self.next.clone(),
            _phantom: PhantomData,
        }
    }
}

// Optional `Copy` implementation when both flows are copyable
impl<Head, Next, InputOk, InputErr> Copy for ResultFlowChain<Head, Next, InputOk, InputErr>
where
    Head: ResultFlow<InputOk, InputErr> + Copy,
    Next: ResultFlow<Head::OutputOk, Head::OutputErr> + Copy,
{
}
