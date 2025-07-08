use std::marker::PhantomData;

/// Step that maps success and error values using functions returning a `Result`.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapBothBindStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> Result<OutputOk, OutputErr>,
    ErrMapperFn: Fn(InputErr) -> Result<OutputOk, OutputErr>,
{
    /// Creates a new [`ResultMapBothBindStep`].
    ///
    /// * `ok_fn` - function to transform the success value
    /// * `err_fn` - function to transform the error value
    pub fn new(ok_fn: OkMapperFn, err_fn: ErrMapperFn) -> Self {
        Self {
            ok_mapper: ok_fn,
            err_mapper: err_fn,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn apply(&self, input_result: Result<InputOk, InputErr>) -> Result<OutputOk, OutputErr> {
        match input_result {
            Ok(ok) => (self.ok_mapper)(ok),
            Err(err) => (self.err_mapper)(err),
        }
    }
}
