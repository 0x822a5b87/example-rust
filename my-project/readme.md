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

## 7.2 Defining Modules to Control Scope and Privacy

### Modules Cheat Sheet

- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually *src/lib.rs* for a library crate or *src/main.rs* for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file *src/garden.rs*
  - In the file *src/garden/mod.rs*