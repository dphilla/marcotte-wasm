// produce a component that has the system layer(s) needed to run a specified program, when
// compiled to Wasm. e.g. an only-what-you-need libc + vfs layer for just the syscalls in a given program

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::fs;
use handlebars::Handlebars;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("libc_main.rs")?;
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("open_template", template)?;

    let full = true;
    let initialize_component_globals;
    let open_body;
    let write_body;
    let close_body;
    let body1;

    let mut args = env::args().skip(1);



    if env::args().len() == 1 {
        println!("\n");
    let multiline_str = r#"_ __ ___   __ _ _ __ ___ ___ | |_| |_ ___     __      ____ _ ___ _ __ ___
 | '_ ` _ \ / _` | '__/ __/ _ \| __| __/ _ \____\ \ /\ / / _` / __| '_ ` _ \
 | | | | | | (_| | | | (_| (_) | |_| ||  __/_____\ V  V / (_| \__ \ | | | | |
 |_| |_| |_|\__,_|_|  \___\___/ \__|\__\___|      \_/\_/ \__,_|___/_| |_| |_|"#;
        println!("{}", multiline_str);
        println!("\n          - Generate System Layers for your Wasm Applications - ");
        println!("\n");

        println!(" marcotte-wasm (alias: mwasm) [args] --[flag]");
        println!("\n");
        println!(" Available arguments: open, read, write, close ");
        println!(" Available flags: --full");
        println!("\n\n More info at: https://github.com/dphilla/marcotte-wasm");
        return Ok(());
    }

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
    } else {
        println!("\nCreating wasm-libc implementation with default stubbed values for: {}\n", call_list);
    }

    if full {
        initialize_component_globals = fs::read_to_string("init_imports.rs")?;
        open_body = fs::read_to_string("wasm_vfs_open.rs")?;
        close_body = fs::read_to_string("wasm_vfs_close.rs")?;
        write_body = fs::read_to_string("wasm_vfs_write.rs")?;
        body1 = fs::read_to_string("file_vfs.rs")?;
    } else {
        initialize_component_globals = "// no system call layer generated".to_string();
        open_body = "0".to_string();
        write_body = "0".to_string();
        close_body = "0".to_string();
        body1 = "0".to_string();
    }
    let data = serde_json::json!({
        "initialize_globals": initialize_component_globals,
        "open_body": open_body,
        "write_body": write_body,
        "close_body": close_body,
        "body1": body1,
    });

    let rendered = handlebars.render("open_template", &data)?;
    fs::write("system_layer.rs", rendered)?;
    Ok(())
}
