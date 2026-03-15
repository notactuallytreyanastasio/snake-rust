# Snake (Rust)

A terminal snake game written in Rust — auto-generated from [Temper](https://temperlang.dev) source code.

## How to Play

**Prerequisites:** Rust 1.71+ with Cargo

```bash
git clone https://github.com/notactuallytreyanastasio/snake-rust.git
cd snake-rust
cd snake-game && cargo run
```

Use **w/a/s/d** keys to steer the snake. No Enter key needed — input is raw mode.

## The Story

This code was not written by a human in Rust. It was written once in [Temper](https://github.com/temperlang/temper) — a programming language that compiles to 6 other languages — and then automatically compiled and published here by CI.

The same snake game exists in 6 languages, all generated from [one Temper source](https://github.com/notactuallytreyanastasio/temper_snake):

| Language | Repository |
|----------|------------|
| JavaScript | [snake-js](https://github.com/notactuallytreyanastasio/snake-js) |
| Python | [snake-python](https://github.com/notactuallytreyanastasio/snake-python) |
| Lua | [snake-lua](https://github.com/notactuallytreyanastasio/snake-lua) |
| **Rust** | **snake-rust** (you are here) |
| C# | [snake-csharp](https://github.com/notactuallytreyanastasio/snake-csharp) |
| Java | [snake-java](https://github.com/notactuallytreyanastasio/snake-java) |

## What We Had to Do to the Compiler

Temper had no way to pause execution or read user input. The only I/O primitive was `console.log()`. To write a game loop that ticks every 200ms and reads keyboard input, we added `sleep()` and `readLine()` to the language itself — across all six compilation backends.

The compiler changes live on the [`do-crimes-to-play-snake`](https://github.com/temperlang/temper/tree/do-crimes-to-play-snake) branch ([PR #376](https://github.com/temperlang/temper/pull/376)).

### The Temper Declaration

Two new `@connected` primitives were added to `std/io` in commit [`0f31c89`](https://github.com/temperlang/temper/commit/0f31c89fabc1c938c6a4d2e72c80af658034aa17):

```temper
@connected("stdSleep")
export let sleep(ms: Int): Promise<Empty> { panic() }

@connected("stdReadLine")
export let readLine(): Promise<String?> { panic() }
```

The `@connected` decorator tells the compiler to replace the `panic()` body with a backend-specific native implementation at compile time.

### What Changed for Rust

Rust uses a custom async runtime (not tokio) based on `Promise<T>`, `PromiseBuilder<T>`, and `SafeGenerator<T>`. The pattern matches the existing `stdNetSend` in `std/net`: create a `PromiseBuilder`, spawn async work via `run_async()`, complete the promise from the worker thread. Three Kotlin compiler files were modified.

**[`RustSupportNetwork.kt`](https://github.com/temperlang/temper/blob/0f31c89fabc1c938c6a4d2e72c80af658034aa17/be-rust/src/commonMain/kotlin/lang/temper/be/rust/RustSupportNetwork.kt)** — registered the connected keys with full crate paths for cross-crate calls:

```diff
 private val netSend = FunctionCall("stdNetSend", "send_request", cloneEvenIfFirst = true)
+private val stdSleep = FunctionCall("stdSleep", "temper_std::io::std_sleep")
+private val stdReadLine = FunctionCall("stdReadLine", "temper_std::io::std_read_line")
```

The full path `temper_std::io::std_sleep` is required because the function lives in the `std` crate but is called from user crates.

**[`RustBackend.kt`](https://github.com/temperlang/temper/blob/0f31c89fabc1c938c6a4d2e72c80af658034aa17/be-rust/src/commonMain/kotlin/lang/temper/be/rust/RustBackend.kt)** — added `"io"` to the support needers set and the Cargo feature list:

```diff
-val stdSupportNeeders = setOf("net", "regex", "temporal")
+val stdSupportNeeders = setOf("io", "net", "regex", "temporal")
```

```diff
 append("[features]\n")
+append("io = []\n")
 append("net = [\"ureq\"]\n")
```

The `io` feature has no external dependencies — only the Rust standard library.

**[`std/io/support.rs`](https://github.com/temperlang/temper/blob/0f31c89fabc1c938c6a4d2e72c80af658034aa17/be-rust/src/commonMain/resources/lang/temper/be/rust/std/io/support.rs)** — the native runtime implementation:

```rust
use std::sync::Arc;
use std::io::BufRead;
use temper_core::{Promise, PromiseBuilder, SafeGenerator};

pub fn std_sleep(ms: i32) -> Promise<()> {
    let pb = PromiseBuilder::new();
    let promise = pb.promise();
    crate::run_async(Arc::new(move || {
        let pb = pb.clone();
        SafeGenerator::from_fn(Arc::new(move |_generator: SafeGenerator<()>| {
            std::thread::sleep(std::time::Duration::from_millis(ms as u64));
            pb.complete(());
            None
        }))
    }));
    promise
}

pub fn std_read_line() -> Promise<Option<Arc<String>>> {
    let pb = PromiseBuilder::new();
    let promise = pb.promise();
    crate::run_async(Arc::new(move || {
        let pb = pb.clone();
        SafeGenerator::from_fn(Arc::new(move |_generator: SafeGenerator<()>| {
            let stdin = std::io::stdin();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(0) => pb.complete(None),
                Ok(_) => {
                    let trimmed = line.trim_end_matches('\n')
                                      .trim_end_matches('\r');
                    pb.complete(Some(Arc::new(trimmed.to_string())));
                }
                Err(_) => pb.complete(None),
            }
            None
        }))
    }));
    promise
}
```

Each function creates a `PromiseBuilder`, spawns a `SafeGenerator` via `run_async`, and returns the `Promise` immediately. The generator runs on a separate thread and completes the promise when done. `std_read_line` returns `Promise<Option<Arc<String>>>` — the `Arc<String>` matches Temper's string representation in the Rust backend.

## How It Works

1. The game logic lives in [`temper_snake`](https://github.com/notactuallytreyanastasio/temper_snake) as `.temper.md` files
2. A custom Temper compiler (branch [`do-crimes-to-play-snake`](https://github.com/temperlang/temper/tree/do-crimes-to-play-snake)) adds the `sleep()` and `readLine()` I/O primitives
3. GitHub Actions builds the compiler, compiles the game for all 6 backends, runs tests
4. If tests pass, the compiled output is automatically pushed to this repo

Every push to the source repo triggers a fresh build. This code is always in sync.

## Source

[notactuallytreyanastasio/temper_snake](https://github.com/notactuallytreyanastasio/temper_snake)
