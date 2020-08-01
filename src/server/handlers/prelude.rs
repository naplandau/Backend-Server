use crate::server::handlers::errors;
use std::result;

pub use crate::server::handlers::errors::Error;
pub use failure::{Fail, ResultExt};
#[allow(dead_code)]
pub type Result<T, E = errors::Error> = result::Result<T, E>;