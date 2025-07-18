use std::marker::PhantomData;

/// Step that passes the success value to a closure and returns its result.
#[derive(Debug, Clone, Copy)]
pub struct OkTapStep<InputOk, OutputOk, TapFn>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    tap: TapFn,
    _phantom: PhantomData<(InputOk, OutputOk)>,
}

impl<TapFn, InputOk, OutputOk> OkTapStep<InputOk, OutputOk, TapFn>
where
    TapFn: Fn(InputOk) -> OutputOk,
{
    /// Creates a new [`OkTapStep`].
    ///
    /// * `tap` - closure that processes the success value
    pub const fn new(tap: TapFn) -> Self {
        Self {
            tap,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_ok: InputOk) -> OutputOk {
        (self.tap)(input_ok)
    }
}
