// produce a component that has the system layer(s) needed to run a specified program, when
// compiled to Wasm. e.g. an only-what-you-need libc + vfs layer for just the syscalls in a given program

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::fs;
use handlebars::Handlebars;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("libc_open.rs")?;
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("open_template", template)?;

    let full = true;
    let initialize_component_globals;
    let body;
    let body1;

    let mut args = env::args().skip(1);
    let mut full = false;
    for arg in args {
        if arg == "--full" {
            full = true;
        } else {
        }
    }

    let mut call_list = "open, write, read, close".to_string();

    if full {
        println!("\nCreating wasm-libc implementations, and relevant system call layer for: {}.\n", call_list);
        initialize_component_globals = fs::read_to_string("init_imports.rs")?;
        body = fs::read_to_string("wasm_vfs_open.rs")?;
        body1 = fs::read_to_string("file_vfs.rs")?;
    } else {
        println!("\nCreating wasm-libc implementation with default stubbed values for: {}\n", call_list);
        initialize_component_globals = "// no system call layer generated".to_string();
        body = "0".to_string();
        body1 = "0".to_string();
    }
    let data = serde_json::json!({
        "initialize_globals": initialize_component_globals,
        "body": body,
        "body1": body1,
    });

    let rendered = handlebars.render("open_template", &data)?;
    fs::write("system_layer.rs", rendered)?;
    Ok(())
}
