use std::fmt::{
    Display,
    Formatter,
    Result,
};

#[derive(Debug, Clone)]
pub enum SystemError {
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "poop") }
}
