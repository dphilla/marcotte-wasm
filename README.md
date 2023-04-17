# marcotte-wasm

Create Layers of host-agnostic system functionality for your WebAssembly (Wasm) application.

## Example Usage

Take this rust code:

```rust
use std::fs;

pub fn scale(ctx: &mut Context) -> Result<&mut Context, Box<dyn std::error::Error>> {
    let filename = "example.txt".to_string();

    // open()
    let mut file = File::open(&filename).expect("Unable to open file");

    let s = "\n\n\n We are interacting with a filesystem!!!\n\n\n";

    let bytes = s.as_bytes();

    // write()
    file.write(&bytes);

    // read()
    let contents = file.read_to_string()?;

    ctx.response().set_body(format!("\nfile: {} ", contents));

    Ok(ctx)
    // close()
}
```
This will not be able to run in Core Wasm, because it relies on underlying libc interfaces, and syscalls provided by a kernel. `marcotte` (aliased: `mwasm`) can create these layers in pure webassembly, and allow your program to run (mostly) as if it were compiled to a native platform.

Running `mwasm open read write close --full` Will generate everything this code needs to run, in a pure WebAssembly Environment. 

## How it works

This cli relies on [wasm-libc](https://github.com/dphilla/wasm-vfs) and [wasm-vfs](https://github.com/dphilla/wasm-libc) to create the libc interface layer, and the "Syscall" functionality all within a consumable Wasm Component. 

## Installation 

clone and `cargo build --release` and drop the generated binary in your /usr/bin (or, your prefered) directory. 

## Limitations 

Many, as **this is a work-in-progress, experimental tool**, but notably:

- Networking Layers 
- Size Limitations
- Complete Filesystem Abstractions

Please see the repo's [issues](https://github.com/dphilla/marcotte-wasm/issues) for more information on these.
