use std::vec::Vec;

pub enum States<T> {
    Initialization,
    Limited,
    Safety(Vec<T>),
    Active,
    Failure,
}

impl<T> States<T> {
    pub fn new() -> Self {
        States::Initialization
    }
}
