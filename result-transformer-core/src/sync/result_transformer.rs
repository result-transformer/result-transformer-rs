pub trait ResultTransformer<InputOk, InputErr> {
    type OutputOk;
    type OutputErr;
    fn transform(&self, result: Result<InputOk, InputErr>) -> Result<Self::OutputOk, Self::OutputErr>;
}