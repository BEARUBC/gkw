use std::collections::BTreeMap;
use std::future::Future;

use crate::{component::component::{Component, Identifier}, system::system::System};
use crate::builder::Builder;

pub type SystemBuilderResult<T> = Result<T, ()>;

struct PermissionSetBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    pub(self) component: Component<M, T, A>,
    pub(self) sendable: Option<Vec<Identifier>>,
}

impl<M, T, A> Builder<PermissionSet<M, T, A>, ()> for PermissionSetBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    fn build(self) -> PermissionSetBuilderResult<System<M, T, A>> { todo!() }
}

impl<M, T, A> PermissionSetBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    #[allow(unused)]
    fn able_to_send(&self, id: Identifier) -> bool {
        if let Some(x) = self.sendable.as_deref() {
            x.iter().any(|&el| el == id)
        } else { true }
    }
}

pub struct SystemBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    components: BTreeMap<Identifier, PermissionSetBuilder<M, T, A>>,
}

impl<M, T, A> Builder<System<M, T, A>, ()> for SystemBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    fn build(self) -> SystemBuilderResult<System<M, T, A>> { todo!() }
}
