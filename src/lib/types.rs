use crate::lib::errors::AppError;
pub(crate) use crate::lib::errors::AppError::NoneError;
use std::result;

pub type Result<T> = result::Result<T, AppError>;
