use crate::error::RandError;

#[link(name = "bcrypt")]
unsafe extern "system" {
    fn BCryptGenRandom(
        hAlgorithm: *mut std::ffi::c_void,
        pbBuffer: *mut u8,
        cbBuffer: u32,
        dwFlags: u32,
    ) -> i32;
}

pub fn rand_api(buf: &mut [u8]) -> Result<(), RandError> {
    let result = unsafe {
        BCryptGenRandom(
            std::ptr::null_mut(),
            buf.as_mut_ptr(),
            buf.len() as u32,
            2,
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(RandError::SyscallFailed { errno: result as i32 })
    }
}