//! result_async_flow_test.rs – Async unit tests for ResultFlow steps

use result_transformer::__internal::*;
#[allow(unused_imports)]
use result_transformer::flow::async_::*;
#[allow(unused_imports)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::test]
async fn map_result_step_transforms_ok_and_err_async() {
    let step = ResultMapBothStep::new(|x: i32| format!("ok={x}"), |e: i32| format!("err={e}"));
    assert_eq!(step.apply_result(Ok(3)).await, Ok("ok=3".to_string()));
    assert_eq!(step.apply_result(Err(9)).await, Err("err=9".to_string()));
}

#[tokio::test]
async fn tap_result_step_transforms_and_logs_async() {
    let step = ResultTapBothStep::new(|x: i32| format!("O{x}"), |e: i32| format!("E{e}"));
    assert_eq!(step.apply_result(Ok(1)).await, Ok("O1".to_string()));
    assert_eq!(step.apply_result(Err(2)).await, Err("E2".to_string()));
}

#[tokio::test]
async fn inspect_result_step_side_effect_only_async() {
    static OK_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ERR_COUNT: AtomicUsize = AtomicUsize::new(0);
    let step = ResultInspectBothStep::new(
        |_| {
            OK_COUNT.fetch_add(1, Ordering::SeqCst);
        },
        |_| {
            ERR_COUNT.fetch_add(1, Ordering::SeqCst);
        },
    );
    assert_eq!(step.apply_result(Ok(123)).await, Ok(123));
    assert_eq!(step.apply_result(Err("fail")).await, Err("fail"));
    assert_eq!(OK_COUNT.load(Ordering::SeqCst), 1);
    assert_eq!(ERR_COUNT.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn noop_result_step_identity_async() {
    let step = ResultNoopStep::<u8, &str>::new();
    assert_eq!(step.apply_result(Ok(100)).await, Ok(100));
    assert_eq!(step.apply_result(Err("err")).await, Err("err"));
}

#[tokio::test]
async fn if_result_step_conditional_branching_async() {
    let even_flow = ResultMapBothStep::new(|x| x / 2, |e| e);
    let odd_flow = ResultMapBothStep::new(|x| x * 3 + 1, |e| e);
    let step = ResultIfStep::new(
        |res: &Result<i32, &str>| matches!(res, Ok(v) if v % 2 == 0),
        even_flow,
        odd_flow,
    );
    assert_eq!(step.apply_result(Ok(6)).await, Ok(3));
    assert_eq!(step.apply_result(Ok(5)).await, Ok(16));
    assert_eq!(step.apply_result(Err("e")).await, Err("e"));
}

#[tokio::test]
async fn result_flow_then_chaining_async() {
    let f1 = ResultMapBothStep::new(|x: i32| x + 1, |e| e);
    let f2 = ResultMapBothStep::new(|x: i32| x * 2, |e| format!("E:{}", e));
    let flow = f1.then_result(f2);
    assert_eq!(flow.apply_result(Ok(4)).await, Ok(10));
    assert_eq!(
        flow.apply_result(Err("oops")).await,
        Err("E:oops".to_string())
    );
}
