use result_transformer::__internal as deps;
use result_transformer::flow::sync::macros::*;
use result_transformer::flow::sync::step::{ErrNoopStep, OkNoopStep, ResultNoopStep};

// Map steps
define_const_ok_map_step!(OK_MAP_STEP, [i32 => i32], |x| x + 1);
define_const_err_map_step!(ERR_MAP_STEP, [&'static str => usize], |e| e.len());
define_const_result_map_step!(RESULT_MAP_STEP, [i32, &'static str => i32, usize], |r| r.map(|v| v + 1).map_err(|e| e.len()));

define_const_result_map_both_step!(RESULT_MAP_BOTH_STEP,
    [i32, &'static str => i32, usize],
    |v| v + 1,
    |e| e.len(),
);

define_const_result_map_both_bind_step!(RESULT_MAP_BOTH_BIND_STEP,
    [i32, &'static str => i32, usize],
    |v| Ok(v + 1),
    |e| Err(e.len()),
);

// Tap steps
define_const_ok_tap_step!(OK_TAP_STEP, [i32 => i32], |v| v);
define_const_err_tap_step!(ERR_TAP_STEP, [&'static str => &'static str], |e| e);
define_const_result_tap_step!(RESULT_TAP_STEP, [i32, &'static str => i32, &'static str], |r| r);

define_const_result_tap_both_step!(RESULT_TAP_BOTH_STEP,
    [i32, &'static str => i32, &'static str],
    |v| v,
    |e| e,
);

define_const_result_tap_both_bind_step!(RESULT_TAP_BOTH_BIND_STEP,
    [i32, &'static str => i32, &'static str],
    |v| Ok(v),
    |e| Err(e),
);

// Inspect steps
define_const_ok_inspect_step!(OK_INSPECT_STEP, i32, |_| {});
define_const_err_inspect_step!(ERR_INSPECT_STEP, &'static str, |_| {});
define_const_result_inspect_step!(RESULT_INSPECT_STEP, [i32, &'static str], |_| {});

define_const_result_inspect_both_step!(RESULT_INSPECT_BOTH_STEP,
    [i32, &'static str],
    |_| {},
    |_| {},
);

// Noop steps
define_const_ok_noop_step!(OK_NOOP_STEP, i32);
define_const_err_noop_step!(ERR_NOOP_STEP, &'static str);
define_const_result_noop_step!(RESULT_NOOP_STEP, [i32, &'static str]);

// Logging steps
#[cfg(feature = "flow-sync-log-step")]
define_const_ok_log_tap_step!(OK_LOG_STEP, i32, deps::log::Level::Info, |v: &i32| format!(
    "{v}"
));
#[cfg(feature = "flow-sync-log-step")]
define_const_err_log_tap_step!(
    ERR_LOG_STEP,
    &'static str,
    deps::log::Level::Warn,
    |e: &&'static str| format!("{e}")
);
#[cfg(feature = "flow-sync-log-step")]
define_const_result_log_tap_step!(RESULT_LOG_STEP, [i32, &'static str], deps::log::Level::Info, |r: &Result<i32,&'static str>| format!("{:?}", r));
#[cfg(feature = "flow-sync-log-step")]
define_const_result_log_tap_both_step!(RESULT_LOG_BOTH_STEP, [i32, &'static str],
    deps::log::Level::Info, |v: &i32| format!("ok={v}"),
    deps::log::Level::Warn, |e: &&'static str| format!("err={e}"),
);

// Conditional steps
define_const_ok_if_step!(OK_IF_STEP, [i32 => i32],
    |v: &i32| *v > 0,
    OkNoopStep::<i32>::new(),
    OkNoopStep::<i32>::new(),
    OkNoopStep<i32>,
    OkNoopStep<i32>,
);

define_const_err_if_step!(ERR_IF_STEP, [&'static str => &'static str],
    |e: &&'static str| !e.is_empty(),
    ErrNoopStep::<&'static str>::new(),
    ErrNoopStep::<&'static str>::new(),
    ErrNoopStep<&'static str>,
    ErrNoopStep<&'static str>,
);

define_const_result_if_step!(RESULT_IF_STEP, [i32, &'static str => i32, &'static str],
    |r: &Result<i32,&'static str>| r.is_ok(),
    ResultNoopStep::<i32, &'static str>::new(),
    ResultNoopStep::<i32, &'static str>::new(),
    ResultNoopStep<i32, &'static str>,
    ResultNoopStep<i32, &'static str>,
);

#[allow(unused)]
use result_transformer::flow::sync::{ErrFlow, OkFlow, ResultFlow};

#[test]
fn const_steps_compile() {
    let _ = OK_MAP_STEP.apply_ok(1);
    let _ = ERR_MAP_STEP.apply_err("e");
    let _ = RESULT_MAP_STEP.apply_result(Ok(2));
    let _ = RESULT_MAP_BOTH_STEP.apply_result(Ok(2));
    let _ = RESULT_MAP_BOTH_BIND_STEP.apply_result(Ok(2));
    let _ = OK_TAP_STEP.apply_ok(3);
    let _ = ERR_TAP_STEP.apply_err("a");
    let _ = RESULT_TAP_STEP.apply_result(Ok(1));
    let _ = RESULT_TAP_BOTH_STEP.apply_result(Ok(1));
    let _ = RESULT_TAP_BOTH_BIND_STEP.apply_result(Ok(1));
    let _ = OK_INSPECT_STEP.apply_ok(1);
    let _ = ERR_INSPECT_STEP.apply_err("b");
    let _ = RESULT_INSPECT_STEP.apply_result(Ok(1));
    let _ = RESULT_INSPECT_BOTH_STEP.apply_result(Ok(1));
    let _ = OK_NOOP_STEP.apply_ok(0);
    let _ = ERR_NOOP_STEP.apply_err("x");
    let _ = RESULT_NOOP_STEP.apply_result(Ok(1));
    #[cfg(feature = "flow-sync-log-step")]
    {
        let _ = OK_LOG_STEP.apply_ok(1);
        let _ = ERR_LOG_STEP.apply_err("e");
        let _ = RESULT_LOG_STEP.apply_result(Ok(1));
        let _ = RESULT_LOG_BOTH_STEP.apply_result(Ok(1));
    }
    let _ = OK_IF_STEP.apply_ok(1);
    let _ = ERR_IF_STEP.apply_err("err");
    let _ = RESULT_IF_STEP.apply_result(Ok(1));
}
