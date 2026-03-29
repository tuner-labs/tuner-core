// Error types for tuner-core

#[derive(Debug)]
pub enum TunerError {
    InvalidInput(String),
    NotFound(String),
    InternalError(String),
}

impl std::fmt::Display for TunerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TunerError {}