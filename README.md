# BongoDB
### A fast, column-store, relational database.

This project is based off of Harvard's [CS165](http://daslab.seas.harvard.edu/classes/cs165/) course, taught by [Stratos Idreos](https://stratos.seas.harvard.edu/).

## Building from Source

The following steps have only been tested on a Debian 10 host machine.

```bash
# Install Rust toolchain using "rustup".
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone this repository.
git clone https://github.com/chenbobby/bongodb.git

# Install the BongoDB client and server.
cargo install --path bongodb

# Run the BongoDB server in the background.
bongodb-server &

# Run the BongoDB client CLU.
bongodb-client
```


## Development Tools

| Tools in CS 165 Environment | Description | BongoDB Equivalent
|-|-|-|
| `perf` | resource usage | keep |
| `valgrind`| memory debugging | keep |
| `strace` | stack debugging | keep |
| `python` | data generation scripts | keep |
| `build-essential`| dependency for Make | keep (dependency for `rustc`) |
| `gcc`| compiler for C | `rustc` and `cargo` |
| `sse4.2-support`| SIMD support | not needed; `rustc` uses LLVM tools for SIMD |
| `pmisc`| utility for managing processes (for example, `killall` by process name) | keep; no alternative found |
| `tmux`| multiplexing terminal windows with docker | ignore |

## Testing

TODO: Rust-style testing

### CS165-style Testing

The tests and data generation scripts from the CS165 course project are available in the [directory](https://github.com/chenbobby/bongodb/tree/master/src) `cs165_project_tests`.
