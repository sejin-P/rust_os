# rust_os

tutorials for making os with rust

[reference](os.phil-opp.com) 

## removing depndencies

1. add #![no_std] in main
2. implement panic instead of basic panic because it depends on operating systems.
3. Disabling Unwinding - Stack Unwinding requires os's functions, so disable it.
4. `start` attribute - In a typical Rust binary that links the standard library, execution starts in a C runtime library called crt0 (“C runtime zero”), which sets up the environment for a C application. This includes creating a stack and placing the arguments in the right registers. The C runtime then invokes the entry point of the Rust runtime, which is marked by the start language item.
-> so we need to define our own entry point. Implementing the start language item wouldn’t help, since it would still require crt0. Instead, we need to overwrite the crt0 entry point directly.
5. Linker errors - By default Rust tries to build an executable that is able to run in your current system environment. -> we should add option when execute `cargo build`
-> `rustup target add thumbv7em-none-eabihf` -> `cargo build --target thumbv7em-none-eabihf`
   (target thumbv7em-none-eabihf which describes an embedded ARM system) (in macOS, use `cargo rustc -- -C link-args="-e __start -static -nostartfiles"`)


## Target specification
see aarch64-rust_os.json

```json
{
   "llvm-target": "aarch64-unknown-none",
   "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
   "arch": "aarch64",
   "target-endian": "little",
   "target-pointer-width": "64",
   "target-c-int-width": "32",
   "os": "none",
   "executables": true,
   // Instead of using the platform’s default linker (which might not support Linux targets), we use the cross-platform LLD linker that is shipped with Rust for linking our kernel.
   "linker-flavor": "ld.lld",
   "linker": "rust-lld",
   // This setting specifies that the target doesn’t support stack unwinding on panic, so instead the program should abort directly. This has the same effect as the panic = "abort" option in our Cargo.toml
   "panic-strategy": "abort",
   // We’re writing a kernel, so we’ll need to handle interrupts at some point. To do that safely, we have to disable a certain stack pointer optimization called the “red zone”, because it would cause stack corruption otherwise.
   "disable-redzone": true,
   // The mmx and sse features determine support for Single Instruction Multiple Data (SIMD) instructions, which can often speed up programs significantly.
   "features": "-mmx,-sse,+soft-float"
}

```

## Bare metal in apple m chip series
```shell
xcode-select --install
brew install gdb picocom qemu
brew install --cask gcc-aarch64-embedded
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils cargo-embed
```


## AARCH64 Bare metal
I ran below command for proper bare metal setting (feature `-mmx`, `+soft-float`, `-sse` is not supported in aarch64)
```shell
rustc -Z unstable-options --print target-spec-json --target aarch64-unknown-none > aarch64-rust_os.json
```

## built in func which links to os's c func
use `compiler_builtins` crate built-in func instead of that os's c func. -> commit 31afdc78f88d6fc867df5ed3fe7898a1a632c771


## Creating a Bootimage
To turn our compiled kernel into a bootable disk image, we need to link it with a bootloader.

Instead of writing my own bootloader, which is a project on its own, I use the bootloader crate.

This crate implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly.
-> commit 5653678f81f2e7d3babafdbadf8440a461683d13

Adding the bootloader as a dependency is not enough to actually create a bootable disk image.

The problem is that we need to link our kernel with the bootloader after compilation, but cargo has no support for post-build scripts.

To solve this problem, we created a tool named bootimage that first compiles the kernel and bootloader, and then links them together to create a bootable disk image.

To install the tool, go into your home directory (or any directory outside your cargo project) and execute the following command in your terminal:

```shell
cargo install bootimage
```

For running bootimage and building the bootloader, you need to have the llvm-tools-preview rustup component installed.

You can do so by executing 
```shell
rustup component add llvm-tools-preview.
```

## Caution
I immediately stopped this project due to cpu architecture,,,

