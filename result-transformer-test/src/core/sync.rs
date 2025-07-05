use result_transformer::sync::{ErrTransformer, OkTransformer, ResultTransformer};

struct SyncTransformer;

impl OkTransformer<i32> for SyncTransformer {
    type OutputOk = i64;
    fn transform_ok(&self, ok: i32) -> Self::OutputOk {
        (ok as i64) * 2
    }
}

impl ErrTransformer<&'static str> for SyncTransformer {
    type OutputErr = String;
    fn transform_err(&self, err: &'static str) -> Self::OutputErr {
        format!("E:{}", err)
    }
}

impl ResultTransformer<i32, &'static str> for SyncTransformer {
    type OutputOk = <Self as OkTransformer<i32>>::OutputOk;
    type OutputErr = <Self as ErrTransformer<&'static str>>::OutputErr;

    fn transform(
        &self,
        result: Result<i32, &'static str>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        match result {
            Ok(v) => Ok(self.transform_ok(v)),
            Err(e) => Err(self.transform_err(e)),
        }
    }
}

#[test]
fn transforms_ok_value() {
    let transformer = SyncTransformer;
    let result = transformer.transform(Ok(3));
    assert_eq!(result, Ok(6));
}

#[test]
fn transforms_err_value() {
    let transformer = SyncTransformer;
    let result = transformer.transform(Err("oops"));
    assert_eq!(result, Err("E:oops".to_string()));
}
