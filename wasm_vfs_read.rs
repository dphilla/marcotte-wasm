  let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
    let bytes_to_read = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, count as usize) };
    process.read(fd as usize, bytes_to_read).unwrap() as ssize_t
