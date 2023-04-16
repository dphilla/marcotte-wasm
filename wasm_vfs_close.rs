  let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
    process.close(fd as i32).unwrap()
