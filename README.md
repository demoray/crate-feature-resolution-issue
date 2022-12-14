# rust dependency feature resolution issue

This is a minimal example of a dependency feature resolution issue in rust.  

Crate `a` and `b` in a single workspace both use the `derive` feature from the crate `clap`.  However, while crate `a` specifies the feature flag in it's Cargo.toml, crate `b` does not specify the feature flag.

# Build results

1. When building `a` and `b` at the same time (via `cargo build`), both crates compile. 
2. When building `a` from the workspace level (via `cargo build --bin a`), `a` compiles.
2. When building `b` from the workspace level (via `cargo build --bin b`), `b` compiles.
4. When buliding `a` from the crate directory (via `cd a, cargo build`), `a` compiles.
5. When buliding `b` from the crate directory (via `cd b, cargo build`), `b` fails to compile with the following error:

```
   Compiling clap v4.0.29
   Compiling b v0.1.0 (/home/USER/src/crate-feature-resolution-issue/b)
error: cannot find derive macro `Parser` in this scope
 --> b/src/main.rs:3:10
  |
3 | #[derive(Parser, Debug)]
  |          ^^^^^^
  |
note: `Parser` is imported here, but it is only a trait, without a derive macro
 --> b/src/main.rs:1:5
  |
1 | use clap::Parser;
  |     ^^^^^^^^^^^^

error[E0599]: no function or associated item named `parse` found for struct `Config` in the current scope
  --> b/src/main.rs:12:26
   |
4  | struct Config {
   | ------------- function or associated item `parse` not found for this struct
...
12 |     let config = Config::parse();
   |                          ^^^^^ function or associated item not found in `Config`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `parse`, perhaps you need to implement one of them:
           candidate #1: `Parser`
           candidate #2: `TypedValueParser`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `b` due to 2 previous errors
```
