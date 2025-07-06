use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Applies different closures to success and error values.
pub struct ResultTapBothStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> OutputOk,
    ErrTapFn: Fn(InputErr) -> OutputErr,
{
    ok_tap: OkTapFn,
    err_tap: ErrTapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultTapBothStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> OutputOk,
    ErrTapFn: Fn(InputErr) -> OutputErr,
{
    pub fn new(ok_fn: OkTapFn, err_fn: ErrTapFn) -> Self {
        Self {
            ok_tap: ok_fn,
            err_tap: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultTapBothStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> OutputOk,
    ErrTapFn: Fn(InputErr) -> OutputErr,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(&self, input: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        match input {
            Ok(v) => Ok((self.ok_tap)(v)),
            Err(e) => Err((self.err_tap)(e)),
        }
    }
}
