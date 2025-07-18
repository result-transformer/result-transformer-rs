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
    fn then_err<Next>(self, next: Next) -> ErrFlowChain<InputErr, Self, Next>
    where
        Self: Sized,
        Next: ErrFlow<Self::OutputErr>,
    {
        ErrFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// Flow that chains two [`ErrFlow`] implementations.
#[derive(Debug, Copy, Clone)]
pub struct ErrFlowChain<InputErr, Head, Next>
where
    Head: ErrFlow<InputErr>,
    Next: ErrFlow<Head::OutputErr>,
{
    /// The first flow in the chain.
    head: Head,
    /// The flow executed after `head`.
    next: Next,
    /// Marker to keep the `InputErr` type parameter.
    _phantom: PhantomData<InputErr>,
}

impl<InputErr, Head, Next> ErrFlowChain<InputErr, Head, Next>
where
    Head: ErrFlow<InputErr>,
    Next: ErrFlow<Head::OutputErr>,
{
    /// Creates a new [`ErrFlowChain`].
    pub fn new(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }

    /// Creates a new [`ErrFlowChain`] usable in const contexts.
    pub const fn new_const(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }
}

impl<InputErr, Head, Next> ErrFlow<InputErr> for ErrFlowChain<InputErr, Head, Next>
where
    Head: ErrFlow<InputErr>,
    Next: ErrFlow<Head::OutputErr>,
{
    type OutputErr = Next::OutputErr;

    fn apply_err(&self, input_err: InputErr) -> Self::OutputErr {
        let result = self.head.apply_err(input_err);
        self.next.apply_err(result)
    }
}
