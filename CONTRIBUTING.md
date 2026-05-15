# Contributing

Thank you for your interest in contributing to anitomy-pure.

## Prerequisites

- Rust stable toolchain (`rustup update stable`)
- `cargo fmt` and `cargo clippy` (included with the toolchain)

## Getting started

```sh
git clone https://github.com/Sans-Atout/anitomy-pure
cd anitomy-pure
cargo build
cargo test
```

## Running the test suite

```sh
cargo test
```

The test suite includes unit tests, integration tests under `tests/`, and data-driven parsing tests in `tests/real_data_*.rs`. A new parsing rule should come with a test case.

## Running benchmarks

```sh
cargo bench
```

Benchmarks live in `benches/` and compare anitomy-pure against anitomy (C++ via FFI), zantetsu, and hunch over 50 real-world filenames.

## Code style

- `cargo fmt` before committing — CI will reject unformatted code
- `cargo clippy -- -D warnings` must pass
- No production dependencies. The design goal is zero-dependency parsing using only `std`.
- No regex. All parsing is done with hand-written O(n) byte scanners.
- No heap allocations in hot paths. Prefer `push_str`/`push` over `format!()`, and static data over per-call `Vec` allocation.

## Submitting changes

1. Fork the repository and create a branch off `master`
2. Write your change and add or update the relevant tests
3. Run `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test`
4. Open a pull request with a short description of what changed and why

## Reporting bugs

Open an issue on GitHub. Include the filename string that was parsed incorrectly and the output you expected.
