use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Step that applies separate closures to success and error values,
/// each returning a `Result`.
pub struct ResultTapBothBindStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    ok_tap: OkTapFn,
    err_tap: ErrTapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultTapBothBindStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    /// Creates a new [`ResultTapBothBindStep`].
    ///
    /// * `ok_fn` - closure for processing the success value
    /// * `err_fn` - closure for processing the error value
    pub fn new(ok_fn: OkTapFn, err_fn: ErrTapFn) -> Self {
        Self {
            ok_tap: ok_fn,
            err_tap: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr> ResultFlow<InputOk, InputErr>
    for ResultTapBothBindStep<OkTapFn, ErrTapFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    /// Implementation of [`ResultFlow::apply_result`].
    fn apply_result(&self, input: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        match input {
            Ok(v) => (self.ok_tap)(v),
            Err(e) => (self.err_tap)(e),
        }
    }
}
