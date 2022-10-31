use std::error::Error;

pub type UnpackError = Box<dyn Error>;
pub type UnpackResult<T> = Result<T, UnpackError>;
