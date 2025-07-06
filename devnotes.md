# Devnotes

run example: `cargo run -- --name Alice`

## build and run

`cargo build --release`

`./target/release/skillet`

`export PATH="$HOME/Documents/WORKSPACE/skillet/target/release:$PATH"`

docs built via hidden command:
`./target/release/skillet --markdown-help`

```shell
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

how to release:

```shell
git checkout main
git pull
git tag -a v1.2.3 -m "Release v1.2.3"
git push origin v1.2.3
```
