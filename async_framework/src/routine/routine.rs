use std::sync::Arc;

use crate::{
    job::Job,
    routine::{
        error::RoutineError,
        routine_builder::RoutineBuilder,
    },
};

pub type RoutineResult<T> = Result<T, RoutineError>;

pub struct Routine<T>
where
T: 'static + ?Sized, {
    jobs: Box<[Arc<Job<T>>]>,
    current_index: usize,
    max_capacity: usize,
}

impl<T> Iterator for Routine<T>
where
T: 'static + ?Sized, {
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
T: 'static + ?Sized, {
    fn from(routine_builder: RoutineBuilder<T>) -> Self {
        let jobs_vec = routine_builder
            .into_inner()
            .into_boxed_slice();
        
        let length = jobs_vec.len();

        Self {
            jobs: jobs_vec,
            current_index: 0usize,
            max_capacity: length,
        }
    }
}
