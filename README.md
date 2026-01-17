# wontonOS
[![test](https://github.com/EvanMu96/wontonOS/actions/workflows/rust.yml/badge.svg)](https://github.com/EvanMu96/wontonOS/actions/workflows/rust.yml)
## Introduction
A repo to learn Rust and basic OSDev.  

## Build

To build this naive operating system, you should install the latest nightly Rust toolchain, including rust-src and llvm-tools-preview components.

```bash
> rustup toolchain install nightly
> rustup component add rust-src
> rustup component add llvm-tools-preview
```

To check your current rust toolchain type and version, you can use command

```bash
> rustc -Vv
```

Moreover, to run the OS in a virtual machine, you should install QEMU with x86 system support

```bash
> sudo apt-get install qemu qemu-system-x86
```

and install bootimage by cargo

```bash
> cargo install bootimage
```

Then, try testing all unit tests

```bash
> cargo test
```

To boot and check screen output

```
> cargo run
```

use any of VNC clients to connect 5900 port for monitoring.

## Reference
https://os.phil-opp.com/
