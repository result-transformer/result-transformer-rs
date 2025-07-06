use std::marker::PhantomData;

/// Defines a transformation applied only to error values.
pub trait ErrFlow<InputErr> {
    /// Resulting error type.
    type OutputErr;

    /// Apply the flow to an error value.
    ///
    /// # Parameters
    /// * `input` - The original error value.
    ///
    /// # Returns
    /// The transformed error value of type [`Self::OutputErr`].
    fn apply_err(&self, input: InputErr) -> Self::OutputErr;

    /// Chain another [`ErrFlow`] after this one.
    ///
    /// The output of the current flow becomes the input of `next`.
    fn then_err<NextFlow>(self, next: NextFlow) -> ErrFlowChain<Self, NextFlow, InputErr>
    where
        Self: Sized,
        NextFlow: ErrFlow<Self::OutputErr>,
    {
        ErrFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// Flow that chains two [`ErrFlow`] implementations.
pub struct ErrFlowChain<FirstFlow, NextFlow, InputErr>
where
    FirstFlow: ErrFlow<InputErr>,
    NextFlow: ErrFlow<FirstFlow::OutputErr>,
{
    /// The first flow in the chain.
    head: FirstFlow,
    /// The flow executed after `head`.
    next: NextFlow,
    /// Marker to keep the `InputErr` type parameter.
    _phantom: PhantomData<InputErr>,
}

impl<FirstFlow, NextFlow, InputErr> ErrFlow<InputErr>
    for ErrFlowChain<FirstFlow, NextFlow, InputErr>
where
    FirstFlow: ErrFlow<InputErr>,
    NextFlow: ErrFlow<FirstFlow::OutputErr>,
{
    type OutputErr = NextFlow::OutputErr;

    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        let result = self.head.apply_err(input_err);
        self.next.apply_err(result)
    }
}
