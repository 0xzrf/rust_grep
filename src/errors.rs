use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("Invalid option provided")]
    InvalidOptionProvided,

    #[error("Failted to match")]
    FailedToMatch,

    #[error("IO error")]
    IOError,
}
