use thiserror::Error;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The result of a file system operation.
pub type ServiceResult<T> = Result<T, ServiceError>;

/// An error that occurred during a file system operation.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Io error.
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    /// Key error.
    #[error("Key error: {0}")]
    KeyError(#[from] zeroutils_key::KeyError),

    /// Config error.
    #[error("Config error: {0}")]
    ConfigError(#[from] zeroutils_config::ConfigError),


    /// Did error.
    #[error("Did error: {0}")]
    DidError(#[from] zeroutils_did_wk::DidError),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates an `Ok` `FsResult` d.
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> ServiceResult<T> {
    Result::Ok(value)
}
