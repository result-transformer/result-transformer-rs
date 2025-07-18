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
    fn then_ok<Next>(self, next: Next) -> OkFlowChain<InputOk, Self, Next>
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
#[derive(Debug, Copy, Clone)]
pub struct OkFlowChain<InputOk, Head, Next>
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

impl<InputOk, Head, Next> OkFlowChain<InputOk, Head, Next>
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

impl<InputOk, Head, Next> OkFlow<InputOk> for OkFlowChain<InputOk, Head, Next>
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
