#![allow(unused)]

use std::sync::Arc;
use std::future::Future;

#[derive(Debug, Clone)]
pub enum RoutineError {
    MaxCapacityReached,
}

impl std::fmt::Display for RoutineError {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result { todo!() }
}

const DEFAULT_MAX_CAPACITY: usize = 10usize;
pub type RoutineResult<T> = Result<T, RoutineError>;

pub enum Job<T>
where
T: 'static + ?Sized, {
    Spacer(u64),
    Lambda(Box<dyn Future<Output = T> +  Unpin>),
}

impl<T> Clone for Job<T> {
    fn clone(&self) -> Self { todo!() }
}

// impl Clone for MyStruct {
//     fn clone(&self) -> Self {
//         MyStruct {
//             field: self.field,
//         }
//     }
// }

unsafe impl<T> Send for Job<T>
where
T: 'static + ?Sized, {}

unsafe impl<T> Sync for Job<T>
where
T: 'static + ?Sized, {}

impl<A, T> From<A> for Job<T>
where
T: 'static + ?Sized,
A: 'static + Future<Output = T> + Unpin, {
    fn from(lambda: A) -> Self { Self::Lambda(Box::new(lambda)) }
}

impl<T> Job<T>
where
T: 'static + ?Sized, {
    pub fn nothing(&self) {}
}

pub struct Routine<T>
where
T: 'static + ?Sized, {
    jobs: Vec<Arc<Job<T>>>,
    current_index: usize,
    max_capacity: usize,
}

impl<T> Routine<T>
where
T: 'static + ?Sized, {
    pub fn new() -> Self {
        Self {
            jobs: vec![],
            current_index: 0usize,
            max_capacity: DEFAULT_MAX_CAPACITY,
        }
    }

    pub fn new_with_capacity(max_capacity: usize) -> Self {
        Self {
            jobs: Vec::with_capacity(max_capacity),
            current_index: 0usize,
            max_capacity,
        }
    }

    pub fn add_spacer(&mut self, spacer: u64) -> RoutineResult<()> { todo!() }

    pub fn add_lambda(&mut self, lambda: Box<dyn Fn() -> ()>) -> RoutineResult<()> { todo!() }

    pub fn remove_job(&mut self, index: usize) -> RoutineResult<()> { todo!() }
}

impl<T> Iterator for Routine<T>
where
T: 'static + ?Sized, {
    type Item = Arc<Job<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        Some({
            let index = self.current_index;
            if self.current_index == (self.max_capacity - 1usize) {
                self.current_index = 0usize;
            };

            self.jobs.get(index).unwrap().clone()
        })
    }
}
