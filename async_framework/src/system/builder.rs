use std::{
    borrow::Cow,
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    builder::Builder,
    component::builder::ComponentBuilder,
    system::{
        System,
        SystemResult,
        error::SystemError,
    },
};

pub struct SystemBuilder<M, R, A, N>(
    Vec<ComponentBuilder<M, R, A, N>>,
)
where
M: 'static + Send,
R: 'static,
A: 'static,;

impl<'a, M, R, A, N> SystemBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static, 
N: Into<Cow<'a, str>>, {
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn push(&mut self, component_builder: ComponentBuilder<M, R, A, N>) { self.0.push(component_builder) }
}

impl<'a, M, R, A, N> Builder<System<M, R, A>, SystemError> for SystemBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static,
N: Into<Cow<'a, str>>, {
    fn build(self) -> SystemResult<System<M, R, A>> { Ok(System::new(self.0)) }
}

impl<'a, M, R, A, N> Deref for SystemBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static,
N: Into<Cow<'a, str>>, {
    type Target = Vec<ComponentBuilder<M, R, A, N>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a, M, R, A, N> DerefMut for SystemBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static,
N: Into<Cow<'a, str>>, {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, M, R, A, N> AsRef<Vec<ComponentBuilder<M, R, A, N>>> for SystemBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static,
N: Into<Cow<'a, str>>, {
    fn as_ref(&self) -> &Vec<ComponentBuilder<M, R, A, N>> { &self.0 }
}

impl<'a, M, R, A, N> AsMut<Vec<ComponentBuilder<M, R, A, N>>> for SystemBuilder<M, R, A, N>
where
M: 'static + Send,
R: 'static,
A: 'static,
N: Into<Cow<'a, str>>, {
    fn as_mut(&mut self) -> &mut Vec<ComponentBuilder<M, R, A, N>> { &mut self.0 }
}
