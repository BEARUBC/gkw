#![allow(unused)]

use std::sync::Arc;
use std::future::Future;

pub enum Job<T>
where
T: 'static + ?Sized, {
    Spacer(u64),
    Lambda(Box<dyn Future<Output = T> + 'static>),
}

impl<T> Clone for Job<T>
where
T: 'static + ?Sized, {
    fn clone(&self) -> Self { todo!() }
}

unsafe impl<T> Send for Job<T>
where
T: 'static + ?Sized, {}

unsafe impl<T> Sync for Job<T>
where
T: 'static + ?Sized, {}

impl<A, T> From<A> for Job<T>
where
T: 'static + ?Sized,
A: 'static + Future<Output = T> + Unpin, {
    fn from(lambda: A) -> Self { Self::Lambda(Box::new(lambda)) }
}
