use std::fmt::{
    Display,
    Formatter,
    Result,
};

#[derive(Debug, Clone)]
pub enum RoutineError {
    MaxCapacityReached,
    InvalidIndex {
        attempted_index: usize,
        maximum_capacity: usize,
    },
}

impl Display for RoutineError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use RoutineError::*;

        match self {
            MaxCapacityReached => write!(f, "cannot put any more jobs into this routine"),
            InvalidIndex {
                attempted_index,
                maximum_capacity,
            } => write!(f, "invalid access into routine - attempted index: {}, maximum capacity: {}", attempted_index, maximum_capacity)
        }
    }
}
