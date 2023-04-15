// writes data from a buffer to a file descriptor
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    {{{write_body}}}
}
