use result_transformer::__internal as deps;
use result_transformer::flow::async_::macros::*;
use result_transformer::flow::async_::{ErrNoopStep, OkNoopStep, ResultNoopStep};

// Map steps
define_const_ok_map_step_async!(OK_MAP_STEP_ASYNC, [i32 => i32], |x| Box::pin(async move { x + 1 }));
define_const_err_map_step_async!(ERR_MAP_STEP_ASYNC, [&'static str => usize], |e| Box::pin(async move { e.len() }));
define_const_result_map_step_async!(RESULT_MAP_STEP_ASYNC, [i32, &'static str => i32, usize], |r| Box::pin(async move { r.map(|v| v + 1).map_err(|e| e.len()) }));

define_const_result_map_both_step_async!(RESULT_MAP_BOTH_STEP_ASYNC,
    [i32, &'static str => i32, usize],
    |v| Box::pin(async move { v + 1 }),
    |e| Box::pin(async move { e.len() }),
);

define_const_result_map_both_bind_step_async!(RESULT_MAP_BOTH_BIND_STEP_ASYNC,
    [i32, &'static str => i32, usize],
    |v| Box::pin(async move { Ok(v + 1) }),
    |e| Box::pin(async move { Err(e.len()) }),
);

// Tap steps
define_const_ok_tap_step_async!(OK_TAP_STEP_ASYNC, [i32 => i32], |v| Box::pin(async move { v }));
define_const_err_tap_step_async!(ERR_TAP_STEP_ASYNC, [&'static str => &'static str], |e| Box::pin(async move { e }));
define_const_result_tap_step_async!(RESULT_TAP_STEP_ASYNC, [i32, &'static str => i32, &'static str], |r| Box::pin(async move { r }));

define_const_result_tap_both_step_async!(RESULT_TAP_BOTH_STEP_ASYNC,
    [i32, &'static str => i32, &'static str],
    |v| Box::pin(async move { v }),
    |e| Box::pin(async move { e }),
);

define_const_result_tap_both_bind_step_async!(RESULT_TAP_BOTH_BIND_STEP_ASYNC,
    [i32, &'static str => i32, &'static str],
    |v| Box::pin(async move { Ok(v) }),
    |e| Box::pin(async move { Err(e) }),
);

// Inspect steps
define_const_ok_inspect_step_async!(OK_INSPECT_STEP_ASYNC, i32, |_| Box::pin(async {}));
define_const_err_inspect_step_async!(ERR_INSPECT_STEP_ASYNC, &'static str, |_| Box::pin(async {}));
define_const_result_inspect_step_async!(RESULT_INSPECT_STEP_ASYNC, [i32, &'static str], |_| Box::pin(async {}));

define_const_result_inspect_both_step_async!(RESULT_INSPECT_BOTH_STEP_ASYNC,
    [i32, &'static str],
    |_| Box::pin(async {}),
    |_| Box::pin(async {}),
);

// Noop steps
define_const_ok_noop_step!(OK_NOOP_STEP_ASYNC, i32);
define_const_err_noop_step!(ERR_NOOP_STEP_ASYNC, &'static str);
define_const_result_noop_step!(RESULT_NOOP_STEP_ASYNC, [i32, &'static str]);

// Logging steps
#[cfg(feature = "flow-async-log-step")]
define_const_ok_log_tap_step!(
    OK_LOG_STEP_ASYNC,
    i32,
    deps::log::Level::Info,
    |v: &i32| format!("{v}")
);
#[cfg(feature = "flow-async-log-step")]
define_const_err_log_tap_step!(
    ERR_LOG_STEP_ASYNC,
    &'static str,
    deps::log::Level::Warn,
    |e: &&'static str| format!("{e}")
);
#[cfg(feature = "flow-async-log-step")]
define_const_result_log_tap_step!(RESULT_LOG_STEP_ASYNC, [i32, &'static str], deps::log::Level::Info, |r: &Result<i32,&'static str>| format!("{:?}", r));
#[cfg(feature = "flow-async-log-step")]
define_const_result_log_tap_both_step!(RESULT_LOG_BOTH_STEP_ASYNC, [i32, &'static str],
    deps::log::Level::Info, |v: &i32| format!("ok={v}"),
    deps::log::Level::Warn, |e: &&'static str| format!("err={e}"),
);

// Conditional steps
define_const_ok_if_step_async!(OK_IF_STEP_ASYNC, [i32 => i32],
    |v: &i32| {
        let val = *v;
        Box::pin(async move { val > 0 })
    },
    OkNoopStep::<i32>::new(),
    OkNoopStep::<i32>::new(),
    OkNoopStep<i32>,
    OkNoopStep<i32>,
);

define_const_err_if_step_async!(ERR_IF_STEP_ASYNC, [&'static str => &'static str],
    |e: &&'static str| {
        let val = *e;
        Box::pin(async move { !val.is_empty() })
    },
    ErrNoopStep::<&'static str>::new(),
    ErrNoopStep::<&'static str>::new(),
    ErrNoopStep<&'static str>,
    ErrNoopStep<&'static str>,
);

define_const_result_if_step_async!(RESULT_IF_STEP_ASYNC, [i32, &'static str => i32, &'static str],
    |r: &Result<i32,&'static str>| {
        let ok = r.is_ok();
        Box::pin(async move { ok })
    },
    ResultNoopStep::<i32, &'static str>::new(),
    ResultNoopStep::<i32, &'static str>::new(),
    ResultNoopStep<i32, &'static str>,
    ResultNoopStep<i32, &'static str>,
);

use deps::tokio;
#[allow(unused)]
use result_transformer::flow::async_::{AsyncErrFlow, AsyncOkFlow, AsyncResultFlow};

#[tokio::test]
async fn const_steps_compile_async() {
    let _ = OK_MAP_STEP_ASYNC.apply_ok_async(1).await;
    let _ = ERR_MAP_STEP_ASYNC.apply_err_async("e").await;
    let _ = RESULT_MAP_STEP_ASYNC.apply_result_async(Ok(1)).await;
    let _ = RESULT_MAP_BOTH_STEP_ASYNC.apply_result_async(Ok(1)).await;
    let _ = RESULT_MAP_BOTH_BIND_STEP_ASYNC
        .apply_result_async(Ok(1))
        .await;
    let _ = OK_TAP_STEP_ASYNC.apply_ok_async(1).await;
    let _ = ERR_TAP_STEP_ASYNC.apply_err_async("e").await;
    let _ = RESULT_TAP_STEP_ASYNC.apply_result_async(Ok(1)).await;
    let _ = RESULT_TAP_BOTH_STEP_ASYNC.apply_result_async(Ok(1)).await;
    let _ = RESULT_TAP_BOTH_BIND_STEP_ASYNC
        .apply_result_async(Ok(1))
        .await;
    let _ = OK_INSPECT_STEP_ASYNC.apply_ok_async(1).await;
    let _ = ERR_INSPECT_STEP_ASYNC.apply_err_async("e").await;
    let _ = RESULT_INSPECT_STEP_ASYNC.apply_result_async(Ok(1)).await;
    let _ = RESULT_INSPECT_BOTH_STEP_ASYNC
        .apply_result_async(Ok(1))
        .await;
    let _ = OK_NOOP_STEP_ASYNC.apply_ok_async(1).await;
    let _ = ERR_NOOP_STEP_ASYNC.apply_err_async("e").await;
    let _ = RESULT_NOOP_STEP_ASYNC.apply_result_async(Ok(1)).await;
    #[cfg(feature = "flow-async-log-step")]
    {
        let _ = OK_LOG_STEP_ASYNC.apply_ok_async(1).await;
        let _ = ERR_LOG_STEP_ASYNC.apply_err_async("e").await;
        let _ = RESULT_LOG_STEP_ASYNC.apply_result_async(Ok(1)).await;
        let _ = RESULT_LOG_BOTH_STEP_ASYNC.apply_result_async(Ok(1)).await;
    }
    let _ = OK_IF_STEP_ASYNC.apply_ok_async(1).await;
    let _ = ERR_IF_STEP_ASYNC.apply_err_async("err").await;
    let _ = RESULT_IF_STEP_ASYNC.apply_result_async(Ok(1)).await;
}
