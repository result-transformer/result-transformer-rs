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
    fn then_err<Next>(self, next: Next) -> ErrFlowChain<Self, Next, InputErr>
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
pub struct ErrFlowChain<Head, Next, InputErr>
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

impl<Head, Next, InputErr> ErrFlowChain<Head, Next, InputErr>
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

impl<Head, Next, InputErr> ErrFlow<InputErr>
    for ErrFlowChain<Head, Next, InputErr>
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

// `Clone` implementation when both flows are cloneable
impl<Head, Next, InputErr> Clone for ErrFlowChain<Head, Next, InputErr>
where
    Head: ErrFlow<InputErr> + Clone,
    Next: ErrFlow<Head::OutputErr> + Clone,
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
impl<FirstFlow, NextFlow, InputErr> Copy for ErrFlowChain<FirstFlow, NextFlow, InputErr>
where
    FirstFlow: ErrFlow<InputErr> + Copy,
    NextFlow: ErrFlow<FirstFlow::OutputErr> + Copy,
{
}
