use std::{
    borrow::Cow,
    future::Future,
};
use tokio::runtime::Builder as TokioBuilder;

use crate::{
    builder::Builder,
    component::component::Component,
    component_builder::builder::ComponentBuilder,
    system_builder::system_builder::SystemBuilder,
    system::error::SystemError,
};

pub type SystemResult<T> = Result<T, SystemError>;

pub struct System<M, T, A>(
    Vec<Component<M, T, A>>,
)
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,;

impl<M, T, A> System<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send, {
    pub(crate) fn new<'a, N>(
        component_builders: Vec<ComponentBuilder<M, T, A, N>>
    ) -> Self
    where
    N: Into<Cow<'a, str>>, {
        Self(
            component_builders
                .into_iter()
                .map(|component_builder| component_builder
                    .build()
                    .unwrap())
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
                    std::thread::sleep(std::time::Duration::from_secs(2u64));
                };
            });

        panic!()
    }
}

impl<'a, M, T, A, N> From<SystemBuilder<M, T, A, N>> for System<M, T, A>
where
M: 'static + Future + Send,
T: 'static + Future + Sized,
A: 'static + Future + Send,
N: Into<Cow<'a, str>>, {
    fn from(system_builder: SystemBuilder<M, T, A, N>) -> Self {
        system_builder
            .build()
            .unwrap()
    }
}
