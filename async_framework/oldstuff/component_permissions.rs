use std::{collections::BTreeMap, future::Future};

use crate::component::component::{Component, Identifier};

pub(crate) struct ComponentPermissions<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    component: Component<M, T, A>,
    sendable: Box<[Identifier]>,
}

impl<M, T, A> ComponentPermissions<M, T, A>
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
