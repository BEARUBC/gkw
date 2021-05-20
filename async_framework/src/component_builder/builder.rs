use std::{borrow::Cow, future::Future};

use crate::{
    component::component::{
        Identifier,
        Component,
    },
    component_builder::error::ComponentBuilderError,
    routine::routine::Routine,
    utils::get_new_id,
};

pub type ComponentBuilderResult<T> = Result<T, ComponentBuilderError>;

pub struct ComponentBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + ?Sized,
A: 'static + Send + Future, {
    pub(crate) id: Identifier,
    pub(crate) name: Option<String>,
    pub(crate) routine: Option<Routine<T>>,
    pub(crate) handler: Option<fn(M) -> A>,
}

impl<M, T, A> ComponentBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + ?Sized,
A: 'static + Send + Future, {
    pub fn new() -> ComponentBuilderResult<Self> {
        get_new_id()
            .map(|id| Self {
                id,
                name: None,
                routine: None,
                handler: None,
            })
            .map_err(ComponentBuilderError::from)
    }

    pub fn set_name<'a, N>(&mut self, name: N)
    where
    N: Into<Cow<'a, str>>, { self.name = Some(name.into().into_owned()); }

    pub fn set_routine(&mut self, routine: Routine<T>) { self.routine = Some(routine) }

    pub fn set_handler(&mut self, handler: fn(M) -> A) { self.handler = Some(handler) }
}
