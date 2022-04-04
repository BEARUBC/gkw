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

    #[display(fmt = "other error")]
    unable_to_serialize_deserialize = 003,

    #[display(fmt = "other error")]
    unable_to_initialize = 004,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    code: ErrorCode,
    msg: Option<String>,
}

impl Error {
    pub fn new<S>(code: ErrorCode, msg: Option<S>) -> Self
    where
        S: Into<String>,
    {
        let msg = msg.map(S::into);
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

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::new(
            ErrorCode::unable_to_serialize_deserialize,
            Some("Unable to serialize/deserialize the given information."),
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
