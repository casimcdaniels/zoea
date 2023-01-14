## Zoea 🦀
A tiny OS written in Rust

### Building the kernel
Add necessary components to your Rust toolchain

```shell
$ rustup component add rust-src
```

Compile for the x86_64 bare metal target defined in project root `cargo build`


### Create bootable disk image
Add necessary components to your Rust toolchain

```shell
$ rustup component add llvm-tools-preview
```

Install **bootimage** tool (`cargo install bootimage`) for linking bootloader to kernel into a single image. Then you will be able to generate bootable image with `cargo bootimage`

#### Booting image with QEMU

```shell
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-zoea/debug/bootimage-zoea.bin
```

or with `cargo run` which will use the **bootimage** runner to launch with QEMU