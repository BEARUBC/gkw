use std::future::Future;

use crate::{component::component::{Component, Identifier,}, system::component_permissions::ComponentPermissions};
use crate::builder::Builder;
use crate::system::{
    system::System,
    
};

pub(crate) type PermissionSetBuilderResult<T> = Result<T, ()>;

pub(crate) struct ComponentPermissionsBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    component: Component<M, T, A>,
    sendable: Option<Vec<Identifier>>,
}

impl<M, T, A> Builder<ComponentPermissions<M, T, A>, ()> for ComponentPermissionsBuilder<M, T, A>
where
M: 'static + Send + Future,
T: 'static + Future + Sized,
A: 'static + Send + Future, {
    fn build(self) -> PermissionSetBuilderResult<ComponentPermissions<M, T, A>> { todo!() }
}
