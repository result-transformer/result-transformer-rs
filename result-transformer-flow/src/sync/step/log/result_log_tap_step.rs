use result_transformer_dependencies::*;

use std::marker::PhantomData;

use crate::sync::flow::ResultFlow;

pub struct ResultLogTapStep<OkType, ErrType> {
    ok_log: Option<LogConfig<OkType>>,
    err_log: Option<LogConfig<ErrType>>,
    _phantom: PhantomData<(OkType, ErrType)>,
}

impl<OkType, ErrType> ResultLogTapStep<OkType, ErrType> {
    pub fn new(ok_log: Option<LogConfig<OkType>>, err_log: Option<LogConfig<ErrType>>) -> Self {
        Self {
            ok_log,
            err_log,
            _phantom: PhantomData,
        }
    }

    pub fn with_ok_log(level: log::Level, format: fn(&OkType) -> String) -> Self {
        Self {
            ok_log: Some(LogConfig::new(level, format)),
            err_log: None,
            _phantom: PhantomData,
        }
    }

    pub fn with_err_log(level: log::Level, format: fn(&ErrType) -> String) -> Self {
        Self {
            ok_log: None,
            err_log: Some(LogConfig::new(level, format)),
            _phantom: PhantomData,
        }
    }

    pub fn with_both_logs(
        ok_level: log::Level,
        ok_format: fn(&OkType) -> String,
        err_level: log::Level,
        err_format: fn(&ErrType) -> String,
    ) -> Self {
        Self {
            ok_log: Some(LogConfig::new(ok_level, ok_format)),
            err_log: Some(LogConfig::new(err_level, err_format)),
            _phantom: PhantomData,
        }
    }

    pub fn silent() -> Self {
        Self {
            ok_log: None,
            err_log: None,
            _phantom: PhantomData,
        }
    }
}

impl<OkType, ErrType> ResultFlow<OkType, ErrType> for ResultLogTapStep<OkType, ErrType> {
    type OutputOk = OkType;
    type OutputErr = ErrType;

    fn apply_result(
        &self,
        input_result: Result<OkType, ErrType>,
    ) -> Result<Self::OutputOk, Self::OutputErr> {
        match &input_result {
            Ok(ok) => {
                if let Some(ok_log) = &self.ok_log {
                    log::log!(ok_log.level, "{}", (ok_log.format)(ok));
                }
            }
            Err(err) => {
                if let Some(err_log) = &self.err_log {
                    log::log!(err_log.level, "{}", (err_log.format)(err));
                }
            }
        }
        input_result
    }
}

pub struct LogConfig<T> {
    pub level: log::Level,
    pub format: fn(&T) -> String,
}

impl<T> LogConfig<T> {
    pub fn new(level: log::Level, format: fn(&T) -> String) -> Self {
        Self { level, format }
    }
}
