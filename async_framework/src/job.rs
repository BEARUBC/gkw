#![allow(unused)]

use std::sync::Arc;
use std::future::Future;

pub enum Job<T>
where
T: 'static + ?Sized, {
    Spacer(u64),
    Lambda(Box<dyn Future<Output = T> +  Unpin>),
}

impl<T> Clone for Job<T> {
    fn clone(&self) -> Self { todo!() }
}

// impl Clone for MyStruct {
//     fn clone(&self) -> Self {
//         MyStruct {
//             field: self.field,
//         }
//     }
// }

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

impl<T> Job<T>
where
T: 'static + ?Sized, {
    pub fn nothing(&self) {}
}
