// For custom error handling
// TODO

use std::error::Error as OtherError;
use std::result;
use thiserror::Error;
use tonic::transport::Error as GrpcError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:?}")]
    Other(#[from] Box<dyn OtherError + Send + Sync>),

    #[error("{0:?}")]
    Grpc(#[from] GrpcError),
}

pub type Result<T> = result::Result<T, Error>;
