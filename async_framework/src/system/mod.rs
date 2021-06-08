pub mod builder;
pub mod error;

use std::{
    borrow::Cow,
    future::Future,
};
use tokio::runtime::Builder as TokioBuilder;

use crate::{builder::Builder, component::{Component, builder::ComponentBuilder}};

use self::{builder::SystemBuilder, error::SystemError};



pub type SystemResult<T> = Result<T, SystemError>;

pub struct System<M, R, A>(
    Box<[Component<M, R, A>]>,
)
where
M: 'static + Send,
R: 'static,
A: 'static + Future,;

impl<M, R, A> System<M, R, A>
where
M: 'static + Send,
R: 'static,
A: 'static + Future, {
    pub(crate) fn new<'a, N>(
        component_builders: Vec<ComponentBuilder<M, R, A, N>>,
    ) -> Self
    where
    N: Into<Cow<'a, str>>, {
        Self(
            component_builders
                .into_iter()
                .map(
                    |component_builder| component_builder
                        .build()
                        .unwrap()
                )
                .collect()
        )
    }

    #[allow(unused)]
    pub fn run(mut self) -> ! {
        self.0.iter_mut().for_each(|component| { component.start().unwrap(); });

        TokioBuilder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    for component in self.0.iter() {
                        component
                            .run_next_job()
                            .expect("unable to run job");
                    };
                };
            });

        panic!()
    }
}

impl<'a, M, R, A, N> From<SystemBuilder<M, R, A, N>> for System<M, R, A>
where
M: 'static + Send,
R: 'static,
A: 'static + Future,
N: Into<Cow<'a, str>>, {
    fn from(system_builder: SystemBuilder<M, R, A, N>) -> Self {
        system_builder
            .build()
            .unwrap()
    }
}

