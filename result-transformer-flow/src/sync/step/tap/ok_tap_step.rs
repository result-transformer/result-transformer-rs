use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

pub struct OkTapStep<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<TapFn, InputOk, OutputOk> OkTapStep<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    pub fn new(tap: TapFn) -> Self {
        Self {
            tap,
            _phantom: PhantomData,
        }
    }
}

impl<TapFn, InputOk, OutputOk> OkFlow<InputOk> for OkTapStep<TapFn, InputOk, OutputOk>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    type OutputOk = OutputOk;

    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        (self.tap)(input_ok)
    }
}
