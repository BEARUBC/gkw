use std::{
    borrow::Cow,
    future::Future,
};

use crate::{
    component::component::{
        Identifier,
        Component,
    },
    component_builder::error::{
        ComponentBuilderError,
        UC,
    },
    routine::routine::Routine,
    utils::get_new_id,
    builder::Builder,
};

pub type ComponentBuilderResult<T> = Result<T, ComponentBuilderError>;

/// # Note
/// Right now, the ComponentBuilder is pretty useless
/// especially since no real logic needs to be done in order to convert from a ComponentBuilder to Component
/// Regardless, this class is preserved for future scalability purposes
pub struct ComponentBuilder<M, T, A, N>
where
M: 'static + Send + Future,
T: 'static + ?Sized,
A: 'static + Send + Future, {
    id: Identifier,
    name: Option<N>,
    routine: Option<Routine<T>>,
    handler: Option<fn(M) -> A>,
}

impl<'a, M, T, A, N> ComponentBuilder<M, T, A, N>
where
M: 'static + Send + Future,
T: 'static + Sized,
A: 'static + Send + Future,
N: Into<Cow<'a, str>>, {
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

    pub fn set_name(&mut self, name: N) { self.name = Some(name); }

    pub fn set_routine(&mut self, routine: Routine<T>) { self.routine = Some(routine) }

    pub fn set_handler(&mut self, handler: fn(M) -> A) { self.handler = Some(handler) }
}

impl<'a, M, T, A, N> Builder<Component<M, T, A>, ComponentBuilderError> for ComponentBuilder<M, T, A, N>
where
M: 'static + Send + Future,
T: 'static + Sized,
A: 'static + Send + Future,
N: Into<Cow<'a, str>>, {
    fn build(mut self) -> ComponentBuilderResult<Component<M, T, A>> {
        if self.name.is_none() {
            Err(ComponentBuilderError::UninitializedComponent(UC::Name))
        } else if self.routine.is_none() {
            Err(ComponentBuilderError::UninitializedComponent(UC::Routine))
        } else if self.handler.is_none() {
            Err(ComponentBuilderError::UninitializedComponent(UC::Handler))
        } else {
            Ok(())
        }
        .map(|()| Component::new(
            self.id,
            self.name
                .take()
                .unwrap(),
                self.routine,
                self.handler,
        ))
    }
}
