use std::{
    sync::Arc,
    future::Future,
};

use crate::{
    builder::Builder,
    job::Job,
    routine::error::RoutineError,
    routine_builder::builder::RoutineBuilder,
};

pub type RoutineResult<T> = Result<T, RoutineError>;

#[derive(Debug)]
pub struct Routine<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    jobs: Box<[Arc<Job<T, M>>]>,
    current_index: usize,
    max_capacity: usize,
}

impl<T, M> Routine<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    pub(crate) fn new(v: Vec<Arc<Job<T, M>>>) -> Self {
        let length = v.len();

        Self {
            jobs: v
                .into_boxed_slice(),
            current_index: 0usize,
            max_capacity: length,
        }
    }
}

impl<T, M> Iterator for Routine<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    type Item = Arc<Job<T, M>>;

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

impl<T, M> From<RoutineBuilder<T, M>> for Routine<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn from(routine_builder: RoutineBuilder<T, M>) -> Self {
        routine_builder
            .build()
            .expect("unable to build")
    }
}
