use std::fmt;
#[allow(dead_code)]
#[derive(Debug)]

pub enum RandError {
    /* プラットフォーム未対応 */
    UnsupportedPlatform { target: String },
    /* 入力が不正だった場合 */
    InvalidInput { detail: String },
    /* エントロピー不足のエラー */
    InsufficientEntropy,
    /* syscallでのエラー */
    SyscallFailed { errno: i32 },
    /* /dev/urandom等読み込み時のエラー */
    FallbackFailed { source: std::io::Error },
    /* プラットフォーム固有のエラー */
    PlatformSpecificError { code: u32, message: String }
}

impl fmt::Display for RandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandError::UnsupportedPlatform { target } => {
                write!(f, "Unsupported platform: {}", target)
            }
            RandError::InvalidInput { detail } => {
                write!(f, "Invalid input: {}", detail)
            }
            RandError::InsufficientEntropy => {
                write!(f, "Insufficient entropy avaiable")
            }
            RandError::SyscallFailed { errno } => {
                write!(f, "System call failed with errno {}", errno)
            }
            RandError::FallbackFailed { source } => {
                write!(f, "Fallback I/O error: {}", source)
            }
            RandError::PlatformSpecificError { code, message } => {
                write!(f, "Platform error (code: {}): {}", code, message)
            }
        }
    }
}

impl std::error::Error for RandError {}