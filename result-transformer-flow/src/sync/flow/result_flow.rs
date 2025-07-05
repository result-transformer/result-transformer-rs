use std::marker::PhantomData;

/// A trait representing a transformation from `Result<InputOk, InputErr>`
/// to `Result<OutputOk, OutputErr>`.
pub trait ResultFlow<InputOk, InputErr> {
    type OutputOk;
    type OutputErr;

    /// Applies the transformation to a `Result` value.
    fn apply_result(&self, input: Result<InputOk, InputErr>) -> Result<Self::OutputOk, Self::OutputErr>;

    /// Chains this flow with another `ResultFlow`, passing the result of this flow
    /// as input to the next.
    fn then_result<NextFlow>(self, next: NextFlow) -> ResultFlowThen<Self, NextFlow, InputOk, InputErr>
    where
        Self: Sized,
        NextFlow: ResultFlow<Self::OutputOk, Self::OutputErr>,
    {
        ResultFlowThen {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// A chained composition of two `ResultFlow`s.
/// The output of `head` becomes the input of `next`.
pub struct ResultFlowThen<Head, Next, InputOk, InputErr>
where
    Head: ResultFlow<InputOk, InputErr>,
    Next: ResultFlow<Head::OutputOk, Head::OutputErr>,
{
    head: Head,
    next: Next,
    _phantom: PhantomData<(InputOk, InputErr)>,
}

impl<Head, Next, InputOk, InputErr> ResultFlow<InputOk, InputErr>
    for ResultFlowThen<Head, Next, InputOk, InputErr>
where
    Head: ResultFlow<InputOk, InputErr>,
    Next: ResultFlow<Head::OutputOk, Head::OutputErr>,
{
    type OutputOk = Next::OutputOk;
    type OutputErr = Next::OutputErr;

    fn apply_result(&self, input_result: Result<InputOk, InputErr>) -> Result<Self::OutputOk, Self::OutputErr> {
        let intermediate = self.head.apply_result(input_result);
        self.next.apply_result(intermediate)
    }
}