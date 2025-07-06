use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

pub struct ResultInspectBothStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType),
    ErrInspectorFn: Fn(&ErrType),
{
    ok_inspector: OkInspectorFn,
    err_inspector: ErrInspectorFn,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
    ResultInspectBothStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType),
    ErrInspectorFn: Fn(&ErrType),
{
    pub fn new(ok_fn: OkInspectorFn, err_fn: ErrInspectorFn) -> Self {
        Self {
            ok_inspector: ok_fn,
            err_inspector: err_fn,
            _phantom: PhantomData,
        }
    }
}

impl<OkInspectorFn, ErrInspectorFn, OkType, ErrType> ResultFlow<OkType, ErrType>
    for ResultInspectBothStep<OkInspectorFn, ErrInspectorFn, OkType, ErrType>
where
    OkInspectorFn: Fn(&OkType),
    ErrInspectorFn: Fn(&ErrType),
{
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result(&self, input: Result<OkType, ErrType>) -> Result<OkType, ErrType> {
        match &input {
            Ok(v) => (self.ok_inspector)(v),
            Err(e) => (self.err_inspector)(e),
        }
        input
    }
}