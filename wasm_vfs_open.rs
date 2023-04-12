
#[cfg_attr(target_arch = "wasm32", export_name = "open")]
#[no_mangle]
pub fn open(path: *const c_char, _: c_int, _: c_int) -> c_int {
    let inode;
    let cstr = unsafe { CString::from_raw(path as *mut c_char) };
    let path_buf = PathBuf::from(cstr.to_str().expect("Failed to convert CString to str"));

    {
        let fs = unsafe { &mut *FILESYSTEM.as_ref().unwrap().get() };
        inode = fs.lookup_inode(&path_buf).expect("Failed to find the inode for the given path");
    }


    let fd;
    {
        let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
        fd = process.open(unsafe { &mut *FILESYSTEM.as_ref().unwrap().get() }, inode).expect("Failed to open the file");
    }
    return fd as c_int;
}
