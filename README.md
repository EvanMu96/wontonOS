# blog_os_workspace
## Introduction
A repo to learn Rust and basic OSDev.  

## Build

To build this naive operating system, your should install the lastest nightly rust toolchain, including rust-src and llvm-tools-preview components.

To check your current rust version, you can use command

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

