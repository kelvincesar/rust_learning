Commands:
- `cargo new --lib`;
- `cargo test`;
- `cargo test -- --test-threads=1` to run tests in serial;
- `cargo test -- --show-output` to show output for the passed cases;
- `cargo test <function_name>` to test a specific function
- `cargo test <module_name>` = `cargo test tests::`
- `cargo test -- --ignore` run ignored tests