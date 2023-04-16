  let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
    let bytes_to_write = unsafe { std::slice::from_raw_parts(buf as *const u8, count as usize) };
    process.write(fd as i32, bytes_to_write).unwrap() as ssize_t
