use std::marker::PhantomData;

pub trait OkFlow<InputOk> {
    type OutputOk;

    fn apply_ok(&self, input: InputOk) -> Self::OutputOk;

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

pub struct OkFlowChain<FirstFlow, NextFlow, InputOk>
where
    FirstFlow: OkFlow<InputOk>,
    NextFlow: OkFlow<FirstFlow::OutputOk>,
{
    head: FirstFlow,
    next: NextFlow,
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
