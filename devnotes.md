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
