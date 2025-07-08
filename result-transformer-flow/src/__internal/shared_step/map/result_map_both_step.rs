use std::marker::PhantomData;

/// Step that maps success and error values using separate functions.
#[derive(Debug, Clone, Copy)]
pub struct ResultMapBothStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> OutputOk,
    ErrMapperFn: Fn(InputErr) -> OutputErr,
{
    ok_mapper: OkMapperFn,
    err_mapper: ErrMapperFn,
    _phantom: PhantomData<(InputOk, InputErr, OutputOk, OutputErr)>,
}

impl<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
    ResultMapBothStep<OkMapperFn, ErrMapperFn, InputOk, InputErr, OutputOk, OutputErr>
where
    OkMapperFn: Fn(InputOk) -> OutputOk,
    ErrMapperFn: Fn(InputErr) -> OutputErr,
{
    /// Creates a new [`ResultMapBothStep`].
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
            Ok(ok) => Ok((self.ok_mapper)(ok)),
            Err(err) => Err((self.err_mapper)(err)),
        }
    }
}
