# Test Crate

This crate contains tests for all other crates in the Hero project.

## Structure

The test crate is organized to mirror the structure of the crates it tests:

```
crates/test/
├── src/
│   ├── translation_handler/
│   │   ├── frontend/
│   │   │   ├── remover_tests.rs
│   │   │   └── getter_tests.rs
│   │   └── backend/
│   │       └── xml/
│   │           └── reader_tests.rs
│   └── local_storage/
│       └── handler_tests.rs
```

## Limitations

Due to visibility constraints in the original crates, some tests may not be able to directly access private functions. There are two approaches to handle this:

1. Create integration tests that only use public APIs
2. Modify the original crates to expose certain functions for testing (using `pub(crate)` or `#[cfg(test)] pub` annotations)

## Running Tests

To run all tests:

```bash
cd crates/test
cargo test
```

To run tests for a specific module:

```bash
cargo test --package test --test translation_handler
```

## Adding New Tests

When adding new tests:

1. Create a new test file in the appropriate directory
2. Add the module to the corresponding mod.rs file
3. Ensure all test modules are marked with `#[cfg(test)]`
4. Only test public APIs unless the original crate has been modified to expose functions for testing