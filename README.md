# Rust Profiler

A lightweight profiling library for Rust programs. With just a couple of macros, you can easily profile execution time and track how many times specific code scopes are entered.

## Features

- **Profile specific code scopes:** Simply wrap a scope with `profile_scope!("identifier")` to track its performance.
- **Automatic metrics:** Collect counts and total execution times.
- **On-demand profiling:** Activate profiling with Cargo features. When the `profiler` feature is not enabled, the macros become no-ops, so there's no runtime overhead.
- **Easy summary:** Print a concise report with `profiler_summary!()`.

## Quick start

Add the following to your `Cargo.toml` dependencies:

```toml
[dependencies]
rust-profiler = "0.1.0"  # Replace with the actual version
```

## Usage

1. Use the profiling macros in your code:

   ```rust
   use rust_profiler::{profile_scope, profiler_summary};

   fn some_function() {
       profile_scope!("some_function_scope");
       // Your logic here...
   }

   fn main() {
       // Profile a specific scope in main
       profile_scope!("main_scope");

       some_function();
       some_function();

       // At the end of your execution, print the profiling summary.
       profiler_summary!();
   }
   ```

2. Compile your project with the profiler feature:

```bash
$ cargo run --features profiler
```

## Example

Check out the `examples/basic.rs` for a full example. To run the example with profiling enabled, use:

```bash
$ cargo run --example basic --release --features profiler

Function:                        Count:    Time:
foo                                2     901.556622ms
 └─ foo-inner                      2     400.689297ms
   └─ foo-inner-inner              2     100.258594ms
   └─ foo-inner-inner-inner        2     100.275474ms
 └─ foo-double                     2     300.517825ms
   └─ foo-double-inner             2     100.274188ms

bar                                1     200.279518ms
 └─ bar-inner                      1     100.137191ms
```

## How It Works

- **Macro `profile_scope!("identifier")`:** Place this macro in any code block you wish to profile. It tracks both the total time spent in the block and the number of times the block is executed.
- **Macro `profiler_summary!()`:** Call this macro at the end of your program (or at any point you wish to see a summary) to print out a table with your profiling results.

When the `profiler` feature is disabled, these macros compile to nothing, ensuring that there is zero overhead for production builds that do not require profiling.


## License

This project is licensed under the MIT License.
