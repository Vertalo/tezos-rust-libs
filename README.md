# Tezos rust libraries (and dependencies)
This repository contains all the rust libraries used in the codebase of tezos/tezos as well as all their dependencies vendored. The purpose is to make a self-contained archive which allows a compilation inside `opam` "sandbox".

## How to change something
 - Add or update libraries
 - Complete or adapt the list in `Cargo.toml`
 - Refresh `Cargo.lock` with `cargo update`
 - Run `cargo vendor` to regenerate `vendor/`
 - Commit everything

## Dependencies tweaks

- librustzcash:
  - change lib path of Cargo.toml to map this repository structure
  - deactivate Orchard parameters loading
- wasmer-2.3.0:
  - Explicit requirement of inkwell 0.1.0-beta.4, related to [this
    issue](https://github.com/wasmerio/wasmer/issues/3565)
  - Fix return type of `warm_memory_data` from `byte_t` to `unsigned char *` to
    partially fix the issue
    [#4908](https://gitlab.com/tezos/tezos/-/issues/4908). See commit 06d78aa4.
