use std::marker::PhantomData;

/// Step that passes the error value to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
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
    /// Creates a new [`ErrTapStep`].
    ///
    /// * `func` - closure that processes the error value
    pub fn new(func: TapFn) -> Self {
        Self {
            tap: func,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_err: InputErr) -> OutputErr {
        (self.tap)(input_err)
    }
}
