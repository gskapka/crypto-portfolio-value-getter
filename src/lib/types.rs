use std::result;

use crate::lib::errors::AppError;
pub(crate) use crate::lib::errors::AppError::NoneError;

pub type Byte = u8;
pub type Result<T> = result::Result<T, AppError>;
