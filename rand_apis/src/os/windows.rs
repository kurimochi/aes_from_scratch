use windows_sys::Win32::Security::Cryptography::BCryptGenRandom;
use crate::error::RandError;

fn rand_api(buf: &mut [u8]) -> Result<(), RandError> {
    let result = unsafe {
        BCryptGenRandom(
            std::ptr::null_mut(),
            buf.as_mut_ptr(),
            buf.len() as u32,
            0,
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(RandError::SyscallFailed { errno: result as i32 })
    }
}