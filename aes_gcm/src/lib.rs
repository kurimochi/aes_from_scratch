pub mod common;
pub mod ghash;
pub mod crypt;
pub mod error;

pub use crypt::{encrypt_gcm, decrypt_gcm};
pub use error::GcmError;