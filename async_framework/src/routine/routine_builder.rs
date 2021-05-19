use std::sync::Arc;

use crate::job::Job;

pub struct RoutineBuilder<T>
where
T: 'static + ?Sized, {
    jobs: Vec<Arc<Job<T>>>,
}

impl<T> RoutineBuilder<T>
where
T: 'static + ?Sized, {
    #[allow(unused)]
    pub fn new() -> Self { Self { jobs: vec![], } }

    #[allow(unused)]
    pub fn new_with_capacity(capacity: usize) -> Self { Self { jobs: Vec::with_capacity(capacity) } }

    pub(crate) fn into_inner(self) -> Vec<Arc<Job<T>>> { self.jobs }
}

impl<T> AsMut<Vec<Arc<Job<T>>>> for RoutineBuilder<T>
where
T: 'static + ?Sized, {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<T>>> {
        &mut self.jobs
    }
}

impl<T> AsRef<Vec<Arc<Job<T>>>> for RoutineBuilder<T>
where
T: 'static + ?Sized, {
    fn as_ref(&self) -> &Vec<Arc<Job<T>>> {
        &self.jobs
    }
}
