use std::marker::PhantomData;

use crate::sync::flow::OkFlow;

/// Step that passes the success value to a closure and returns its result.
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
    /// Creates a new [`OkTapStep`].
    ///
    /// * `tap` - closure that processes the success value
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

    /// Implementation of [`OkFlow::apply_ok`].
    fn apply_ok(&self, input_ok: InputOk) -> Self::OutputOk {
        (self.tap)(input_ok)
    }
}
