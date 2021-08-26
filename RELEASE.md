# Setting up

Install `cargo-release` with:

```shell
cargo install cargo-release
```

# Configuration

Configuration goes in the `[package.metadata.release]` section of `Cargo.toml`

# Releasing

```shell
# First, choose `major`, `minor`, or `patch` for the level to release

# Next, run the command in dry-run mode
$ cargo release -vv --dry-run LEVEL

# Then do it for real with the same level
$ cargo release LEVEL
```
