# blog_os_workspace
[![test](https://github.com/EvanMu96/blog_os_workspace/actions/workflows/rust.yml/badge.svg)](https://github.com/EvanMu96/blog_os_workspace/actions/workflows/rust.yml)
## Introduction
A repo to learn Rust and basic OSDev.  

## Build

To build this naive operating system, your should install the lastest nightly rust toolchain, including rust-src and llvm-tools-preview components.

```bash
> rustup toochain install nightly
> rustup component add rust-src
> rustup component add llvm-tools-preview
```

To check your current rust toolchain type and version, you can use command

```bash
> rustc -Vv
```

Moreover, to run the os in a virtual machine, you should install qemu with x86 system sypport

```bash
> sudo apt-get install qemu qemu-system-x86
```

and install bootimage by cargo

```bash
> cargo install bootimage
```

Then, try testing all unit test

```bash
> cargo test
```

To boot and check screen printing

```
> cargo run
```

use any of VNC clients to connect 5900 port for monitoring.

## Reference
https://os.phil-opp.com/

