use std::{
    ops::{
        DerefMut,
        Deref,
    },
    sync::Arc,
    future::Future,
};

use crate::{
    job::Job,
    builder::Builder,
    routine::routine::Routine,
};

pub type RoutineBuilderResult<T> = Result<T, ()>;

pub struct RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    jobs: Vec<Arc<Job<T, M>>>,
}

impl<T, M> RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    #[allow(unused)]
    pub fn new() -> Self { Self { jobs: vec![], } }

    #[allow(unused)]
    pub fn new_with_capacity(capacity: usize) -> Self { Self { jobs: Vec::with_capacity(capacity) } }
}

impl<T, M> AsMut<Vec<Arc<Job<T, M>>>> for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<T, M>>> { &mut self.jobs }
}

impl<T, M> AsRef<Vec<Arc<Job<T, M>>>> for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn as_ref(&self) -> &Vec<Arc<Job<T, M>>> { &self.jobs }
}

impl<T, M> Deref for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    type Target = Vec<Arc<Job<T, M>>>;

    fn deref(&self) -> &Self::Target { &self.jobs }
}

impl<T, M> DerefMut for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.jobs }
}

impl<T, M> Builder<Routine<T, M>, ()> for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn build(self) -> RoutineBuilderResult<Routine<T, M>> {
        Ok(Routine::new(self.jobs))
    }
}
