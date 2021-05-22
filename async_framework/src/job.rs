#![allow(unused)]

use std::sync::Arc;
use std::future::Future;

pub enum Job<T>
where
T: 'static + Future + Sized, {
    Spacer(u64),
    Lambda(Box<fn() -> T>),
}

impl<T> Clone for Job<T>
where
T: 'static + Future + Sized, {
    fn clone(&self) -> Self { todo!() }
}

unsafe impl<T> Send for Job<T>
where
T: 'static + Future + Sized, {}

unsafe impl<T> Sync for Job<T>
where
T: 'static + Future + Sized, {}

impl<T> From<fn() -> T> for Job<T>
where
T: 'static + Future + Sized, {
    fn from(lambda: fn() -> T) -> Self { Self::Lambda(Box::new(lambda)) }
}
