use std::marker::PhantomData;

/// Step that applies separate closures to the success and error values.
#[derive(Debug, Clone, Copy)]
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
    /// Creates a new [`ResultTapBothStep`].
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

    pub(crate) fn apply(&self, input_result: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        match input_result {
            Ok(ok) => Ok((self.ok_tap)(ok)),
            Err(err) => Err((self.err_tap)(err)),
        }
    }
}
