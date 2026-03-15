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

## What Is This?

This code was not written by a human in Rust. It was written once in [Temper](https://temperlang.dev) — a programming language that compiles to JavaScript, Python, Lua, Rust, Java, and C# — and then automatically compiled to Rust and published here by CI.

Temper had no way to pause execution or read input. The only I/O was `console.log()`. To play snake, we had to add `sleep(ms)` and `readLine()` to the language itself — modifying the Temper compiler across all six backends.

## What Changed in the Temper Compiler for Rust

Rust uses a custom async runtime (not tokio) based on `Promise<T>`, `PromiseBuilder<T>`, and `SafeGenerator<T>`. Each I/O operation spawns work on a separate thread and completes a promise.

**Compiler changes:**
- `RustSupportNetwork.kt`: `FunctionCall` entries with cross-crate paths (`"temper_std::io::std_sleep"`, `"temper_std::io::std_read_line"`)
- `RustBackend.kt`: `"io"` added to `stdSupportNeeders` and Cargo features. A second dependency-detection pass was added because connected functions bypass the import system — `RustTranslator` tracks `usedSupportFunctionPaths`, and `RustBackend` scans them after translation to inject the `temper-std` dependency with correct features
- `RustTranslator.kt`: records connected function paths in `usedSupportFunctionPaths` during `translateCallExpressionForSupportCode`

**Runtime** (`std/io/support.rs`): `std_sleep` spawns a thread that calls `thread::sleep` and completes a `PromiseBuilder`. `std_read_line` uses `libc::tcgetattr`/`tcsetattr` for raw terminal mode on Unix, reading a single byte from stdin. The `io` Cargo feature depends on `libc`.

## All 6 Backends

The same snake game exists in 6 languages, all generated from [one Temper source](https://github.com/notactuallytreyanastasio/temper_snake):

| Language | Repository |
|----------|------------|
| JavaScript | [snake-js](https://github.com/notactuallytreyanastasio/snake-js) |
| Python | [snake-python](https://github.com/notactuallytreyanastasio/snake-python) |
| Lua | [snake-lua](https://github.com/notactuallytreyanastasio/snake-lua) |
| Rust | [snake-rust](https://github.com/notactuallytreyanastasio/snake-rust) |
| C# | [snake-csharp](https://github.com/notactuallytreyanastasio/snake-csharp) |
| Java | [snake-java](https://github.com/notactuallytreyanastasio/snake-java) |

## Source

- Game source: [notactuallytreyanastasio/temper_snake](https://github.com/notactuallytreyanastasio/temper_snake)
- Compiler branch: [`do-crimes-to-play-snake`](https://github.com/temperlang/temper/tree/do-crimes-to-play-snake) ([PR #376](https://github.com/temperlang/temper/pull/376))

---

*Auto-generated from commit [`a39098b041de1619f89cf75b20e1b9365fc7ee3c`](https://github.com/notactuallytreyanastasio/temper_snake/commit/a39098b041de1619f89cf75b20e1b9365fc7ee3c)*
