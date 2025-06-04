pub mod error;
pub mod os;
pub mod util;
pub mod tests;

pub use error::RandError;
pub use util::{gen_random, zeroize};