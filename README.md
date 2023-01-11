## Zoea 🦀
A tiny OS written in Rust

### Build
Compile for a bare metal target e.g.

`cargo build --target thumbv7em-none-eabihf`


Alternatively, compile for a given host system

**Linux:**
`cargo rustc -- -C link-arg=-nostartfiles`

**Windows:**
`cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`

**macOS:**
`cargo rustc -- -C link-args="-e __start -static -nostartfiles"`