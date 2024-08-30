# GETTING STARTED

## Hello, Cargo!

```bash
# create project with cargo
cargo new hello_cargo --bin

# building a cargo project
cd hello_cargo
cargo build

# running a cargo project
./target/debug/hello_cargo 

# or
cargo run

# checks your code to make sure it compiles but doesn't produce an executable.
cargo check
```

### Building for Release

```bash
cargo build --release
```

