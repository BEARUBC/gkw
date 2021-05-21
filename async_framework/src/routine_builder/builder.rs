use std::{
    ops::{
        DerefMut,
        Deref,
    },
    sync::Arc,
};

use crate::{
    job::Job,
    builder::Builder,
    routine::routine::Routine,
};

pub type RoutineBuilderResult<T> = Result<T, ()>;

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
}

impl<T> AsMut<Vec<Arc<Job<T>>>> for RoutineBuilder<T>
where
T: 'static + ?Sized, {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<T>>> { &mut self.jobs }
}

impl<T> AsRef<Vec<Arc<Job<T>>>> for RoutineBuilder<T>
where
T: 'static + ?Sized, {
    fn as_ref(&self) -> &Vec<Arc<Job<T>>> { &self.jobs }
}

impl<T> Deref for RoutineBuilder<T>
where
T: 'static + ?Sized, {
    type Target = Vec<Arc<Job<T>>>;

    fn deref(&self) -> &Self::Target { &self.jobs }
}

impl<T> DerefMut for RoutineBuilder<T>
where
T: 'static + ?Sized, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.jobs }
}

impl<T> Builder<Routine<T>, ()> for RoutineBuilder<T> {
    fn build(self) -> RoutineBuilderResult<Routine<T>> {
        Ok(Routine::new(self.jobs))
    }
}
