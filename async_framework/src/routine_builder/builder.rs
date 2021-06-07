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

#[derive(Debug)]
pub struct RoutineBuilder<T, M>(
    Vec<Arc<Job<T, M>>>,
)
where
T: 'static + Future + Sized,
M: 'static + Future + Send,;

impl<T, M> RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn push(&mut self, job: Job<T, M>) { self.0.push(Arc::new(job)) }
}

impl<T, M> Builder<Routine<T, M>, ()> for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn build(self) -> RoutineBuilderResult<Routine<T, M>> {
        Ok(Routine::new(self.0))
    }
}

impl<T, M> Deref for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    type Target = Vec<Arc<Job<T, M>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T, M> DerefMut for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<T, M> AsRef<Vec<Arc<Job<T, M>>>> for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn as_ref(&self) -> &Vec<Arc<Job<T, M>>> { &self.0 }
}

impl<T, M> AsMut<Vec<Arc<Job<T, M>>>> for RoutineBuilder<T, M>
where
T: 'static + Future + Sized,
M: 'static + Future + Send, {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<T, M>>> { &mut self.0 }
}
