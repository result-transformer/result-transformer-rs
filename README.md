# result-transformer
[![crates.io](https://img.shields.io/crates/v/result-transformer.svg)](https://crates.io/crates/result-transformer)
[![docs.rs](https://docs.rs/result-transformer/badge.svg)](https://docs.rs/result-transformer)

> **Composable, type‑safe transforms for `Result<T, E>` – with matching *sync* and *async* APIs.**

`result-transformer` lets you describe how a `Result` should be rewritten **without coupling that logic to your domain
types or execution model**.
Implement a tiny trait – or let a one‑line macro do it for you – and drop the transformer anywhere you need it.
Current version is `0.0.1`. The API is not yet stable and may change.

## Table of contents
- [Why use result-transformer?](#why-use-result-transformer)
- [Crate overview](#crate-overview)
- [Quick start](#quick-start)
  - [Add the dependency](#1-add-the-dependency)
  - [A minimal synchronous transformer](#2-a-minimal-synchronous-transformer)
  - [Async example](#async-example)
  - [Manual implementation (no macros)](#manual-implementation-no-macros)
  - [3. (Optional) Build a transformer via a flow](#3-optional-build-a-transformer-via-a-flow)
- [Feature matrix](#feature-matrix)
- [Project status](#project-status)
- [License](#license)
- [Contributing](#contributing)

---

## Why use result‑transformer?

* **Decoupled by design** – success (`OkTransformer`) and error (`ErrTransformer`) paths are isolated, keeping concerns
  clear and testable.
* **Symmetric sync / async** – every concept exists in twin flavours so you can switch models with zero friction.
* **Macro‑assisted ergonomics** – declarative `impl_*_transformer!` and `impl_*_transformer_via_*_flow!`
  macros generate the boring glue while you focus on behaviour.
* **Optional “flow / step” DSL** – chain lightweight *steps* into a *flow* when you want a quick inline pipeline.
  (It’s a helper, not a requirement.)
* **Tiny & feature‑gated** – pull in only the capabilities and dependencies you actually use.

---

## Crate overview

| crate                               | role                                                                           | key features                       |
| ----------------------------------- | ------------------------------------------------------------------------------ | ---------------------------------- |
| **result‑transformer‑core**         | foundational traits & “raw” macros                                             | `sync`, `async`, `*_macros`        |
| **result‑transformer‑flow**         | *optional* DSL made of *steps* that can be turned into transformers via macros | logging, map / tap / inspect steps |
| **result‑transformer‑macros**       | reserved placeholder for future procedural macros                              | –                                  |
| **result‑transformer‑dependencies** | consolidates external crates behind feature‑flags (`tokio`, `log`, …)          | –                                  |
| **result‑transformer‑test**         | integration & doc‑tests you can read as real‑world recipes                     | –                                  |

---

## Quick start

### 1. Add the dependency

```toml
# Cargo.toml
result-transformer = { version = "0.0.1", features = ["core-sync", "core-sync-macros"] }
```

### 2. A minimal synchronous transformer

```rust
use result_transformer::sync::macros::*;

struct DoublePrefix;

/// doubles on success and prefixes errors – nothing more.
impl_ok_transformer! {
    impl_for      = DoublePrefix,
    input_ok      = i32,
    output_ok     = i32,
    transform_ok  = |v| v * 2
}

impl_err_transformer! {
    impl_for       = DoublePrefix,
    input_err      = &'static str,
    output_err     = String,
    transform_err  = |e| format!("E:{e}")
}

impl_result_transformer_via_self_parts! {
    impl_for  = DoublePrefix,
    input_ok  = i32,
    input_err = &'static str
}

assert_eq!(DoublePrefix.transform(Ok(21)), Ok(42));
assert_eq!(DoublePrefix.transform(Err("no")), Err("E:no".into()));
```

Everything above works exactly the same inside `async` contexts – just enable the `core-async` feature and swap the module path to `async_`.

#### Manual implementation (no macros)

This approach is handy when macros are unavailable or you need finer-grained control.

```rust
use result_transformer::sync::{
    OkTransformer, ErrTransformer, ResultTransformer
};

struct PlainTransformer;

impl OkTransformer<i32> for PlainTransformer {
    type OutputOk = i32;
    fn transform_ok(&self, ok: i32) -> Self::OutputOk {
        ok * 2
    }
}

impl ErrTransformer<&'static str> for PlainTransformer {
    type OutputErr = String;
    fn transform_err(&self, err: &'static str) -> Self::OutputErr {
        format!("E:{err}")
    }
}

impl ResultTransformer<i32, &'static str> for PlainTransformer {
    type OutputOk = <Self as OkTransformer<i32>>::OutputOk;
    type OutputErr = <Self as ErrTransformer<&'static str>>::OutputErr;

    fn transform(&self,
                 result: Result<i32, &'static str>)
                 -> Result<Self::OutputOk, Self::OutputErr> {
        match result {
            Ok(v) => Ok(self.transform_ok(v)),
            Err(e) => Err(self.transform_err(e)),
        }
    }
}

use result_transformer::sync::ResultTransformer; // needed to access the `transform` method

assert_eq!(PlainTransformer.transform(Ok(21)), Ok(42));
assert_eq!(PlainTransformer.transform(Err("no")), Err("E:no".into()));
```

The manual route makes the separation of concerns crystal‑clear – **each trait lives on its own** – and it requires
*zero* compiler magic.  In larger codebases you’ll probably reach for the macros to avoid repetition, but both styles
interoperate seamlessly.

---

## 3. (Optional) Build a transformer *via* a flow

> **Note:** To use flows, enable the `flow-sync` and `flow-sync-macros` features.

When you do want a small pipeline the *flow / step* DSL has your back.
The example below chains two *steps* and then turns the resulting flow into a reusable transformer with
`impl_result_transformer_via_result_flow!`.

```rust
use result_transformer::flow::sync::{
    step::map::ResultMapBothStep,
    macros::impl_result_transformer_via_result_flow,
};

// any value implementing `ResultFlow` can be used – a single step is already a valid flow!
let flow = ResultMapBothStep::new(|ok: i32| ok * 2, |err: &str| err.to_owned())
    // chaining the steps with `.then_result`
    .then_result(ResultMapBothStep::new(|ok| ok + 1, |err| format!("E:{err}")));

struct ViaFlow;

impl_result_transformer_via_result_flow! {
    impl_for   = ViaFlow,
    input_ok   = i32,
    input_err  = &'static str,
    output_ok  = i32,
    output_err = String,
    flow       = flow
}
```

---

## Feature matrix

| feature               | effect |
| --------------------- | --------------------------------------------------------- |
| `default`             | enables `core-sync` |
| `core-sync`           | enables the synchronous core API |
| `core-sync-macros`    | helper macros for the synchronous model |
| `core-sync-all`       | `core-sync` + `core-sync-macros` |
| `core-async`          | enables the asynchronous core API |
| `core-async-macros`   | helper macros for the asynchronous model |
| `core-async-all`      | `core-async` + `core-async-macros` |
| `core-all`            | all features from `result-transformer-core` |
| `flow-sync`           | enables the synchronous *flow / step* DSL |
| `flow-sync-macros`    | helper macros for synchronous flows |
| `flow-sync-log-step`  | adds a logging step for synchronous flows |
| `flow-sync-all`       | `flow-sync` + `flow-sync-log-step` + `flow-sync-macros` |
| `flow-async`          | enables the asynchronous *flow / step* DSL |
| `flow-async-macros`   | helper macros for asynchronous flows |
| `flow-async-log-step` | adds a logging step for asynchronous flows |
| `flow-async-all`      | `flow-async` + `flow-async-log-step` + `flow-async-macros` |
| `flow-all`            | all features from `result-transformer-flow` |
| `sync-all`            | `core-sync-all` + `flow-sync-all` |
| `async-all`           | `core-async-all` + `flow-async-all` |
| `all`                 | every feature in this crate |

Pick exactly what you need and keep compile times down.
Only `core-sync` is enabled by default. Combine features as needed.

> Some features automatically enable their dependencies.
> For example, `flow-async-macros` will automatically enable `flow-async` and `core-async`.
> You can still list them explicitly if you want to be more precise.

Example: enabling the async execution model, flow DSL, and flow macros:

```toml
result-transformer = { version = "0.0.1", features = ["flow-async-macros"] }
```

Or, if you prefer to list everything explicitly:

```toml
result-transformer = { version = "0.0.1", features = ["core-async", "flow-async", "flow-async-macros"] }
```

---

## Project status

This project is currently in early development (`0.0.1`) and **not yet stable**.
APIs — especially in the `flow` crate — are subject to change.

---

## License

Licensed under either of

* **MIT** license *(LICENSE-MIT)*
* **Apache‑2.0** license *(LICENSE-APACHE)*

at your option.

---

## Contributing

Bug reports, feature requests, and pull requests are always welcome.

> I'm still new to GitHub and not very confident in English.
> For now, I'm doing my best with the help of ChatGPT,
> so I might misunderstand something. Thanks for your understanding!

---