use std::result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarsError {
    #[error("internal engine error: {0}")]
    Internal(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("configuration error: {0}")]
    Config(String),
}

pub type Result<T> = result::Result<T, MarsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_error_conversion() {
        let e: MarsError = std::fs::File::open("no_such_file").unwrap_err().into();
        assert!(matches!(e, MarsError::Io(_)));
    }

    #[test]
    fn internal_error_message() {
        let err = MarsError::Internal("boom".into());
        assert_eq!(format!("{err}"), "internal engine error: boom");
    }
}