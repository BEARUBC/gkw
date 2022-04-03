use std::sync::PoisonError;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use derive_more::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::zero_prefixed_literal)]
pub enum ErrorCode {
    #[allow(dead_code)]
    #[display(fmt = "other error")]
    other = 000,

    #[allow(dead_code)]
    #[display(fmt = "other error")]
    unable_to_grab_lock = 001,

    #[display(fmt = "other error")]
    unable_to_transition = 002,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    code: ErrorCode,
    msg: Option<&'static str>,
}

impl Error {
    pub fn new(code: ErrorCode, msg: Option<&'static str>) -> Self {
        Self { code, msg }
    }
}

impl<T> From<PoisonError<RwLockReadGuard<'static, T>>> for Error {
    fn from(_: PoisonError<RwLockReadGuard<'static, T>>) -> Self {
        Self::new(
            ErrorCode::unable_to_grab_lock,
            Some(
                "Something went wrong while trying to grab an immutable copy of the requested lock.",
            ),
        )
    }
}

impl<T> From<PoisonError<RwLockWriteGuard<'static, T>>> for Error {
    fn from(_: PoisonError<RwLockWriteGuard<'static, T>>) -> Self {
        Self::new(
            ErrorCode::unable_to_grab_lock,
            Some("Something went wrong while trying to grab a mutable copy of the requested lock."),
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ERROR {:#03}: {}]", self.code as u8, self.code)?;

        if let Some(msg) = &self.msg {
            write!(f, " {}", msg)?;
        };

        Ok(())
    }
}
