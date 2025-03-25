use crate::core::error::ShieldError;

pub mod delete;
pub mod init;
pub mod list;

pub type Result<T> = std::result::Result<T, ShieldError>;
