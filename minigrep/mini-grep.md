# mini-grep

> mini `grep` in Rust.

## Accepting Command Line Arguments

- ` search-string` the string to search for
- `search-file.txt` the file path
- `--` two hyphens to indicate the following arguments are for our program rather than cargo

```bash
cargo run -- search-string search-file.txt
```

### [Separation of Concerns for Binary Projects](https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects)

 Rust community has developed guidelines for splitting the separate concerns of a binary program when `main` starts getting large. This process has the following steps:

- Split your program into a *main.rs* and a *lib.rs* and move your program’s logic to *lib.rs*.
- As long as your command line parsing logic is small, it can remain in *main.rs*.
- When the command line parsing logic starts getting complicated, extract it from *main.rs* and move it to *lib.rs*.

The responsibilities that remain in the `main` function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a `run` function in *lib.rs*
- Handling the error if `run` returns an error