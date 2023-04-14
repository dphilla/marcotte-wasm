// produce a component that has the system layer(s) needed to run a specified program, when
// compiled to Wasm
// e.g. a only-what-you-need libc + vfs layer for just the syscalls in a given program

use std::env;
//fn main() {
    //let mut args = env::args().skip(1);
    //let mut full = false;
    //for arg in args {
        //if arg == "--full" {
            //full = true;
        //} else {
            //println!("Argument: {}", arg);
        //}
    //}
    //if full {
        //println!("Full flag is set");
    //}
//}
//

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::fs;

use handlebars::Handlebars;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("libc_open.rs")?;

    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("open_template", template)?;

    let full = true;

    // START HERE: with open
    // 1. system_layer.rs gets generated, and that is what uses wasi-vfs under the hood (from a
    // crate)
    // 2. make exportable, to work in a scale func
    // 2. specifically open, read, write, close, to start
    // 3. crates for wasm_vfs, marcotte_wasm

    let initialize_component_globals;
    let body;

    if full {
      // inject underlying "Syscalls" --
      initialize_component_globals = fs::read_to_string("init_imports.rs")?;
      body = fs::read_to_string("wasm_vfs_open.rs")?;
    } else {
      // return default values, effectively "stubbing" these lib calls
      initialize_component_globals = "// no system call layer generated".to_string();
      body = "0".to_string();
    };

    let data = serde_json::json!({
        "initialize_globals": initialize_component_globals,
        "body": body,
    });

    let rendered = handlebars.render("open_template", &data)?;
    fs::write("system_layer.rs", rendered)?;

    Ok(())
}
