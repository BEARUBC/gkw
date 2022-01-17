use std::io;
use std::sync::MutexGuard;
use std::sync::PoisonError;

use crate::kernel::Kernel;

pub trait ToIoError {
    fn to_io_error(self, error: &'static str) -> io::Error;
}

impl<'a> ToIoError for PoisonError<MutexGuard<'a, Kernel>> {
    fn to_io_error(self, error: &'static str) -> io::Error {
        io::Error::new(io::ErrorKind::Other, error)
    }
}
