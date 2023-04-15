// closes a directory stream
pub extern "C" fn closedir(dirp: *mut c_void) -> c_int {
    {{{close_body}}}
}
