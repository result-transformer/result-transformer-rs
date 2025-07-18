use std::marker::PhantomData;

/// Defines a transformation applied only to successful values.
pub trait OkFlow<InputOk> {
    /// Resulting success type.
    type OutputOk;

    /// Apply the flow to a success value.
    ///
    /// # Parameters
    /// * `input` - The original success value.
    ///
    /// # Returns
    /// The transformed success value of type [`Self::OutputOk`].
    fn apply_ok(&self, input: InputOk) -> Self::OutputOk;

    /// Chain another [`OkFlow`] after this one.
    ///
    /// The output of the current flow is used as the input of `next`.
    fn then_ok<Next>(self, next: Next) -> OkFlowChain<Self, Next, InputOk>
    where
        Self: Sized,
        Next: OkFlow<Self::OutputOk>,
    {
        OkFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// Flow that chains two [`OkFlow`] implementations.
pub struct OkFlowChain<Head, Next, InputOk>
where
    Head: OkFlow<InputOk>,
    Next: OkFlow<Head::OutputOk>,
{
    /// The first flow in the chain.
    head: Head,
    /// The flow executed after `head`.
    next: Next,
    /// Marker to keep the `InputOk` type parameter.
    _phantom: PhantomData<InputOk>,
}

impl<Head, Next, InputOk> OkFlowChain<Head, Next, InputOk>
where
    Head: OkFlow<InputOk>,
    Next: OkFlow<Head::OutputOk>,
{
    /// Creates a new [`OkFlowChain`].
    pub fn new(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }

    /// Creates a new [`OkFlowChain`] usable in const contexts.
    pub const fn new_const(head: Head, next: Next) -> Self {
        Self {
            head,
            next,
            _phantom: PhantomData,
        }
    }
}

impl<Head, Next, InputOk> OkFlow<InputOk> for OkFlowChain<Head, Next, InputOk>
where
    Head: OkFlow<InputOk>,
    Next: OkFlow<Head::OutputOk>,
{
    type OutputOk = Next::OutputOk;

    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        let result = self.head.apply_ok(input_ok);
        self.next.apply_ok(result)
    }
}

// `Clone` implementation when both flows are cloneable
impl<Head, Next, InputOk> Clone for OkFlowChain<Head, Next, InputOk>
where
    Head: OkFlow<InputOk> + Clone,
    Next: OkFlow<Head::OutputOk> + Clone,
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
impl<Head, Next, InputOk> Copy for OkFlowChain<Head, Next, InputOk>
where
    Head: OkFlow<InputOk> + Copy,
    Next: OkFlow<Head::OutputOk> + Copy,
{
}
