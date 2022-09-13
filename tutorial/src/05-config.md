# Configuration

- Create a new Rust project
- In your `Cargo.toml` file, add `rusty_engine` to your `[dependencies]` section:

```toml
# In your [dependencies] section of Cargo.toml
rusty_engine = "5.2.0"
```

### (Optional) Make `dev` profile act like `release`

If you don't want to have remember to constantly add `--release` to your `cargo build` and `cargo run` commands, you can add this config section to your `Cargo.toml` to make your `dev` profile act like the `release` profile:

```toml
[profile.dev]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
incremental = false
codegen-units = 16
```
