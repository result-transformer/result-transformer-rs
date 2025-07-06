use std::marker::PhantomData;

use crate::sync::flow::ErrFlow;

/// Executes a closure with the error value and returns its result.
pub struct ErrTapStep<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> OutputErr,
{
    tap: TapFn,
    _phantom: PhantomData<(InputErr, OutputErr)>,
}

impl<TapFn, InputErr, OutputErr> ErrTapStep<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> OutputErr,
{
    pub fn new(func: TapFn) -> Self {
        Self {
            tap: func,
            _phantom: PhantomData,
        }
    }
}

impl<TapFn, InputErr, OutputErr> ErrFlow<InputErr> for ErrTapStep<TapFn, InputErr, OutputErr>
where
    TapFn: Fn(InputErr) -> OutputErr,
{
    type OutputErr = OutputErr;

    fn apply_err(&self, input_ok: InputErr) -> Self::OutputErr {
        (self.tap)(input_ok)
    }
}
