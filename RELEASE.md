# Setting up

Install `cargo-release` with:

```shell
cargo install cargo-release
```

# Configuration

Configuration goes in `release.toml`

# Releasing

```shell
# First, choose `major`, `minor`, or `patch` for the level to release

# Next, run the command in dry-run mode
$ cargo release -v LEVEL

# Then do it for real with the same level
$ cargo release --execute LEVEL

# Then publish an updated tutorial
$ script/publish_tutorial
```
