# BongoDB
### A fast, column-store, relational database.

This project is based off of Harvard's [CS165](http://daslab.seas.harvard.edu/classes/cs165/) course, taught by [Stratos Idreos](https://stratos.seas.harvard.edu/).

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

### CS165-style Testing

The data generation scripts from the CS165 course project are copied to the directory `cs165_project_tests`.

#### Setup
1. Install Python 3.
1. Create a Python 3 virtual environment in the directory `cs165_project_tests`.
1. Install Python dependencies into the virtual environment.
    - Python dependencies are listed in the file `cs165_project_tests/requirements.txt`.

A handy setup script completes all the setup steps above, but it has only been tested on a Debian 10 operating system.

It is important to run the setup script from inside the `cs165_project_test` directory. You can run the setup script with the following two commands.
```bash
cd cs165_project_tests
./setup.sh
```

#### Generate Test Data
