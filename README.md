# `elsa` - ~~A CLI tool to list items in a directory~~ It's `ls`. Literally in the name.

## Usage

List the contents of the current directory:

```bash
$ elsa
```

List the contents of a specific directory:

```bash
$ elsa /path/to/directory
```

## Installation

### Prerequisites

- Rust
- Cargo (Rust's package manager)

### Using `cargo install`

```bash
cargo install elsa-cli
```

And now you can run `elsa` from anywhere (well, assuming you have `~/.cargo/bin`
in your `PATH`)!

## Building from source

`elsa` is a Rust program, so you'll need to install the Rust toolchain to build
it. You can do this using [rustup](https://rustup.rs/).

```bash
$ git clone https://github.com/interrrp/elsa
$ cd elsa
$ cargo build --release
```

The binary will be located at `target/release/elsa`.

## License

`elsa` is licensed under the MIT license. See the [LICENSE](LICENSE) file for
more details.
