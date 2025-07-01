pub trait OkTransformer<InputOk> {
    type OutputOk;
    fn transform_ok(&self, ok: InputOk) -> Self::OutputOk;
}
