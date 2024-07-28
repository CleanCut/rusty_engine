# Releasing

Ensure the latest version of `cargo-release` is installed:

```shell
cargo install cargo-release
```

Choose what level this release should be: `major`, `minor`, or `patch`. Do a dry run at that level.

```shell
cargo release -v LEVEL
```

Once that looks good, do the actual release.

```shell
cargo release --execute LEVEL
```

Then publish the tutorial for the new version.


```shell
script/publish_tutorial
```

# Configuration

Configuration goes in `release.toml`
