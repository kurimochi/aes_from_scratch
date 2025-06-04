use crate::error::RandError;
use std::{fs::File, io::Read};

pub fn urandom_fallback(buf: &mut [u8]) -> Result<(), RandError> {
    let mut file = File::open("/dev/urandom")
        .map_err(|e| RandError::FallbackFailed { source: e })?;
    file.read_exact(buf)
        .map_err(|e| RandError::FallbackFailed { source: e })?;
    Ok(())
}