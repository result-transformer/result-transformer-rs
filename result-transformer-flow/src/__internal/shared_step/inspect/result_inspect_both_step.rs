use std::marker::PhantomData;

/// Step that inspects both the success and error values without modifying them.
#[derive(Debug, Clone, Copy)]
pub struct ResultInspectBothStep<OkType, ErrType, OkInspectorFn, ErrInspectorFn>
where
    OkInspectorFn: Fn(&OkType),
    ErrInspectorFn: Fn(&ErrType),
{
    ok_inspector: OkInspectorFn,
    err_inspector: ErrInspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType, OkInspectorFn, ErrInspectorFn>
    ResultInspectBothStep<OkType, ErrType, OkInspectorFn, ErrInspectorFn>
where
    OkInspectorFn: Fn(&OkType),
    ErrInspectorFn: Fn(&ErrType),
{
    /// Creates a new [`ResultInspectBothStep`].
    ///
    /// * `ok_fn` - inspector for the success value
    /// * `err_fn` - inspector for the error value
    pub const fn new(ok_fn: OkInspectorFn, err_fn: ErrInspectorFn) -> Self {
        Self {
            ok_inspector: ok_fn,
            err_inspector: err_fn,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        match &input_result {
            Ok(ok) => (self.ok_inspector)(ok),
            Err(err) => (self.err_inspector)(err),
        }
        input_result
    }
}
