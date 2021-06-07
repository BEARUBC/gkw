use std::{
    borrow::Cow,
    future::Future,
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    component_builder::builder::ComponentBuilder,
    system::system::System,
    builder::Builder,
};

pub type SystemBuilderResult<T> = Result<T, ()>;

pub struct SystemBuilder<M, T, A, N>(
    Vec<ComponentBuilder<M, T, A, N>>
)
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,;

impl<'a, M, T, A, N> SystemBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }
}

impl<'a, M, T, A, N> Builder<System<M, T, A>, ()> for SystemBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn build(self) -> SystemBuilderResult<System<M, T, A>> { Ok(System::new(self.0)) }
}

impl<'a, M, T, A, N> Deref for SystemBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    type Target = Vec<ComponentBuilder<M, T, A, N>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a, M, T, A, N> DerefMut for SystemBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, M, T, A, N> AsRef<Vec<ComponentBuilder<M, T, A, N>>> for SystemBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn as_ref(&self) -> &Vec<ComponentBuilder<M, T, A, N>> { &self.0 }
}

impl<'a, M, T, A, N> AsMut<Vec<ComponentBuilder<M, T, A, N>>> for SystemBuilder<M, T, A, N>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn as_mut(&mut self) -> &mut Vec<ComponentBuilder<M, T, A, N>> { &mut self.0 }
}
