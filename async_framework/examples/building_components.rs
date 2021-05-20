use tokio::runtime::Builder;
use std::{future::Future, pin::Pin, sync::Arc, task::{Context, Poll}, thread, time::Duration};

use async_framework::{component::component::Component, component_builder::builder::ComponentBuilder, job::Job, routine_builder::builder::RoutineBuilder};

struct MS;

impl Future for MS {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> { println!("MS poll method called"); Poll::Pending  }
}

async fn handler(message: MS) -> () {
    message.await;
}

fn main() -> () {
    let j1 = Arc::new(Job::Spacer(1u64));
    let j2 = Arc::new(Job::Lambda(Box::new(async {
        println!("hello, world!")
    })));

    let mut routine_builder = RoutineBuilder::new_with_capacity(2usize);
    let vec = routine_builder.as_mut();
    
    vec.push(j1);
    vec.push(j2);

    let routine = routine_builder.into();

    let mut component_builder = ComponentBuilder::new().unwrap();
    component_builder.set_name("custom");
    component_builder.set_routine(routine);
    component_builder.set_handler(handler)
}
