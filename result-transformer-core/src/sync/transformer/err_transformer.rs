pub trait ErrTransformer<InputErr> {
    type OutputErr;
    fn transform_err(&self, err: InputErr) -> Self::OutputErr;
}
