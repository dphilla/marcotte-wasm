use std::io;
use std::path::Path;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::Once;
use std::cell::UnsafeCell;
use std::path::PathBuf;
use wasm_vfs::{Process, FileSystem, File};

static mut FILESYSTEM: Option<UnsafeCell<FileSystem>> = None;
static mut PROCESS: Option<UnsafeCell<Process>> = None;
static INIT: Once = Once::new();

pub fn initialize_globals() {
    INIT.call_once(|| {
        unsafe {
            FILESYSTEM = Some(UnsafeCell::new(FileSystem::new()));
            PROCESS = Some(UnsafeCell::new(Process::new()));
        }
    });
}
