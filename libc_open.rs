{{{initialize_globals}}}
// opens a file or device and returns a file descriptor
#[cfg_attr(target_arch = "wasm32", export_name = "open")] // can add apply to all as needed
#[no_mangle]
pub extern "C" fn open(path: *const c_char, oflag: c_int, mode: c_int) -> c_int {
    {{{body}}}

}

{{{body1}}}
