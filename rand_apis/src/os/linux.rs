use crate::error::RandError;
use crate::os::fallback::urandom_fallback;

unsafe extern "C" {
    fn getrandom(buf: *mut u8, buflen: usize, flags: u32) -> isize;
}

pub fn rand_api(buf: &mut [u8]) -> Result<(), RandError> {
    let ret = unsafe {
        getrandom(
            buf.as_mut_ptr(),
            buf.len(),
            0
        )
    };
    if ret == buf.len() as isize {
        Ok(())
    } else {
        urandom_fallback(buf)
    }
}