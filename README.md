# vcrash: A Virtual Crash Tool

This is a simple implementation of generating virtual application crash.
It runs a process and kills it after a random time. It is useful for testing
your app's logging mechanism. Note that it does not simulate power failures.
Therefore, write reordering is not available to check with `vcrash`.

## Build

For all platforms, Rust is required to build the project:

### Obtaining Rust Compiler

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Compile

```sh
git clone https://github.com/NVSL/vcrash.git
cd vcrash
cargo build --release
```

You may optionally link the executable in your home bin directory.

```sh
cp target/release/vcrash ~/bin/
```

## Usage

```sh
$ vcrash
usage: vcrash [rand max] [prog] [args ...]
```

* `rand max`: maximum waiting time before the crash in nanoseconds
* `prog`: path to the executable
* `args`: arguments to be passed to the app

### Example

```sh
$ vcrash 1000000 cp path/to/src path/to/dst
crashing after 57661 nanoseconds ...
```

Enjoy!
