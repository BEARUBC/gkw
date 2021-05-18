use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::component::component::MutexError;

#[derive(Debug, Clone)]
pub enum ComponentError {
    IdError,
}

impl Display for ComponentError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use ComponentError::*;

        match self {
            IdError => write!(f, "invalid first item to double"),
        }
    }
}

impl<'a> From<MutexError<'a>> for ComponentError {
    fn from(_: MutexError) -> Self { todo!() }
}
