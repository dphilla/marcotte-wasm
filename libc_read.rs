// reads data from a file descriptor into a buffer
pub extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    {{{read_body}}}
}
