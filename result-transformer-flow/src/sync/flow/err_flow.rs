use std::marker::PhantomData;

pub trait ErrFlow<InputErr> {
    type OutputErr;

    fn apply_err(&self, input: InputErr) -> Self::OutputErr;

    fn then_err<NextFlow>(self, next: NextFlow) -> ErrFlowThen<Self, NextFlow, InputErr>
    where
        Self: Sized,
        NextFlow: ErrFlow<Self::OutputErr>,
    {
        ErrFlowThen {
            head: self,
            next,
            _phantom: PhantomData,
        }
    }
}

pub struct ErrFlowThen<FirstFlow, NextFlow, InputErr>
where
    FirstFlow: ErrFlow<InputErr>,
    NextFlow: ErrFlow<FirstFlow::OutputErr>,
{
    head: FirstFlow,
    next: NextFlow,
    _phantom: PhantomData<InputErr>,
}

impl<FirstFlow, NextFlow, InputErr> ErrFlow<InputErr> for ErrFlowThen<FirstFlow, NextFlow, InputErr>
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