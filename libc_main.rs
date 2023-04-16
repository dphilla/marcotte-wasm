{{{initialize_globals}}}

type size_t  = usize;
type ssize_t  = isize;

// opens a file or device and returns a file descriptor
#[cfg_attr(target_arch = "wasm32", export_name = "open")] // can add apply to all as needed
#[no_mangle]
pub extern "C" fn open(path: *const c_char, oflag: c_int, mode: c_int) -> c_int {
  {{{open_body}}}
}

// reads data from a file descriptor into a buffer
// writes data from a buffer to a file descriptor
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
  {{{write_body}}}
}

// closes a file descriptor
pub extern "C" fn close(fd: c_int) -> c_int {
  {{{close_body}}}
}

{{{body1}}}
