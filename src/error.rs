use std::sync::MutexGuard;
use std::sync::PoisonError;

use derive_more::Display;

use crate::kernel::Kernel;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorCode {
    #[allow(dead_code)]
    #[display(fmt = "other error")]
    other = 000,

    #[allow(dead_code)]
    #[display(fmt = "other error")]
    unable_to_grab_lock = 001,
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

impl<'a> From<PoisonError<MutexGuard<'a, Kernel>>> for Error {
    fn from(_: PoisonError<MutexGuard<'a, Kernel>>) -> Self {
        Self::new(
            ErrorCode::unable_to_grab_lock,
            Some("Something went wrong while trying to grab the `KERNEL` lock."),
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
