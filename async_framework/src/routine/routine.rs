use std::sync::Arc;
use std::future::Future;

use crate::{
    job::Job,
    routine::error::RoutineError,
    routine_builder::builder::RoutineBuilder,
    builder::Builder,
};

pub type RoutineResult<T> = Result<T, RoutineError>;

pub struct Routine<T>
where
T: 'static + Future + Sized, {
    jobs: Box<[Arc<Job<T>>]>,
    current_index: usize,
    max_capacity: usize,
}

impl<T> Routine<T>
where
T: 'static + Future + Sized, {
    pub(crate) fn new(v: Vec<Arc<Job<T>>>) -> Self {
        let length = v.len();

        Self {
            jobs: v
                .into_boxed_slice(),
            current_index: 0usize,
            max_capacity: length,
        }
    }
}

impl<T> Iterator for Routine<T>
where
T: 'static + Future + Sized, {
    type Item = Arc<Job<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.jobs
            .get(self.current_index)
            .expect("self.current_index went out of scope... something went terribly wrong")
            .clone());
        
        self.current_index = if self.current_index == (self.max_capacity - 1usize) {
            0usize
        } else {
            self.current_index + 1usize
        };

        result
    }
}

impl<T> From<RoutineBuilder<T>> for Routine<T>
where
T: 'static + Future + Sized, {
    fn from(routine_builder: RoutineBuilder<T>) -> Self {
        routine_builder
            .build()
            .expect("unable to build")
    }
}
