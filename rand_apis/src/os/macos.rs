use crate::error::RandError;
use crate::os::fallback::urandom_fallback;

unsafe extern "C" {
    fn SecRandomCopyBytes(rnd: *const std::ffi::c_void, count: usize, bytes: *mut u8) -> i32;
}

const KSEC_RANDOM_DEFAULT: *const std::ffi::c_void = 0 as *const _;

pub fn rand_api(buf: &mut [u8]) -> Result<(), RandError> {
    let result = unsafe {
        SecRandomCopyBytes(
            KSEC_RANDOM_DEFAULT,
            buf.len(),
            buf.as_mut_ptr(),
        )
    };

    if result == 0 {
        Ok(())
    } else {
        urandom_fallback(buf)
    }
}