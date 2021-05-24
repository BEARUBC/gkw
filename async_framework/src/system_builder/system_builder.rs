use std::collections::BTreeMap;
use std::future::Future;

use crate::{component::component::Component};
use crate::{component::component::{Identifier}, system::system::System};
use crate::builder::Builder;

pub type SystemBuilderResult<T> = Result<T, ()>;

pub struct SystemBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    components: BTreeMap<Identifier, Component<M, T, A>>,
}

impl<M, T, A> Builder<System<M, T, A>, ()> for SystemBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    fn build(self) -> SystemBuilderResult<System<M, T, A>> { todo!() }
}
