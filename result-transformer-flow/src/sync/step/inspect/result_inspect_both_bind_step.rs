use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

/// Inspects success and error values and returns new `Result`s.
pub struct ResultInspectBothBindStep<
    OkInspectorFn,
    ErrInspectorFn,
    OkType,
    ErrType,
    OutputOk,
    OutputErr,
> where
    OkInspectorFn: Fn(&OkType) -> Result<OutputOk, OutputErr>,
    ErrInspectorFn: Fn(&ErrType) -> Result<OutputOk, OutputErr>,
{
    ok_inspector: OkInspectorFn,
    err_inspector: ErrInspectorFn,
    _phantom: PhantomData<(OkType, ErrType, OutputOk, OutputErr)>,
}

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType, OutputOk, OutputErr>
    ResultInspectBothBindStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType, OutputOk, OutputErr>
where
    OkInspectorFn: Fn(&OkType) -> Result<OutputOk, OutputErr>,
    ErrInspectorFn: Fn(&ErrType) -> Result<OutputOk, OutputErr>,
{
    pub fn new(ok_fn: OkInspectorFn, err_fn: ErrInspectorFn) -> Self {
        Self {
            ok_inspector: ok_fn,
            err_inspector: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType, OutputOk, OutputErr>
    ResultFlow<OkType, ErrType>
    for ResultInspectBothBindStep<
        OkInspectorFn,
        ErrInspectorFn,
        OkType,
        ErrType,
        OutputOk,
        OutputErr,
    >
where
    OkInspectorFn: Fn(&OkType) -> Result<OutputOk, OutputErr>,
    ErrInspectorFn: Fn(&ErrType) -> Result<OutputOk, OutputErr>,
{
    type OutputOk = OutputOk;
    type OutputErr = OutputErr;

    fn apply_result(&self, input: Result<OkType, ErrType>) -> Result<OutputOk, OutputErr> {
        match input {
            Ok(v) => (self.ok_inspector)(&v),
            Err(e) => (self.err_inspector)(&e),
        }
    }
}
