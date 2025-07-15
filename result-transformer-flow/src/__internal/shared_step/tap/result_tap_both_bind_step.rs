use std::marker::PhantomData;

/// Step that applies separate closures to success and error values,
/// each returning a `Result`.
#[derive(Debug, Clone, Copy)]
pub struct ResultTapBothBindStep<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    ok_tap: OkTapFn,
    err_tap: ErrTapFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn>
    ResultTapBothBindStep<InputOk, InputErr, OutputOk, OutputErr, OkTapFn, ErrTapFn>
where
    OkTapFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrTapFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    /// Creates a new [`ResultTapBothBindStep`].
    ///
    /// * `ok_fn` - closure for processing the success value
    /// * `err_fn` - closure for processing the error value
    pub const fn new(ok_fn: OkTapFn, err_fn: ErrTapFn) -> Self {
        Self {
            ok_tap: ok_fn,
            err_tap: err_fn,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(
        &self,
        input_result: Result<InputOk, InputErr>,
    ) -> Result<OutputOk, OutputErr> {
        match input_result {
            Ok(ok) => (self.ok_tap)(ok),
            Err(err) => (self.err_tap)(err),
        }
    }
}
