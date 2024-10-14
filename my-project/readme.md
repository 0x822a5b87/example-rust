# crates, packages, modules, path

## create crate and packages

```bash
# create new crate
cargo new my-project --lib && cd my-project

# create package in crate my-project
mkdir util && cd util && cargo init --lib
cd ..

# create package in crate app
mkdir app && cd app && cargo init --bin

# add dependency
cargo add --path ../my-util
```