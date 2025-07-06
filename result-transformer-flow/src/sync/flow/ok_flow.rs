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
    fn then_ok<NextFlow>(self, next: NextFlow) -> OkFlowChain<Self, NextFlow, InputOk>
    where
        Self: Sized,
        NextFlow: OkFlow<Self::OutputOk>,
    {
        OkFlowChain {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

/// Flow that chains two [`OkFlow`] implementations.
pub struct OkFlowChain<FirstFlow, NextFlow, InputOk>
where
    FirstFlow: OkFlow<InputOk>,
    NextFlow: OkFlow<FirstFlow::OutputOk>,
{
    /// The first flow in the chain.
    head: FirstFlow,
    /// The flow executed after `head`.
    next: NextFlow,
    /// Marker to keep the `InputOk` type parameter.
    _phantom: PhantomData<InputOk>,
}

impl<FirstFlow, NextFlow, InputOk> OkFlow<InputOk> for OkFlowChain<FirstFlow, NextFlow, InputOk>
where
    FirstFlow: OkFlow<InputOk>,
    NextFlow: OkFlow<FirstFlow::OutputOk>,
{
    type OutputOk = NextFlow::OutputOk;

    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        let result = self.head.apply_ok(input_ok);
        self.next.apply_ok(result)
    }
}
