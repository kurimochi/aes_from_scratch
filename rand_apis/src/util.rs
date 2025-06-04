use crate::error::RandError;
use crate::os::rand_api;

pub fn gen_random(buf: &mut[u8]) -> Result<(), RandError> {
    if buf.is_empty() {
        return Err(RandError::InvalidInput {
            detail: "buffer must not be empty".to_string(),
        });
    }

    rand_api(buf)
}

pub fn zeroize(buf: &mut[u8]) {
    for byte in buf.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
}