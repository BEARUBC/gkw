use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{
        Context,
        Poll
    }
};

use async_framework::{
    component_builder::builder::ComponentBuilder,
    job::Job,
    routine_builder::builder::RoutineBuilder,
    builder::Builder,
};

struct MS;

impl Future for MS {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MS poll method called");
        Poll::Pending
    }
}

async fn handler(message: MS) -> () {
    message.await;
}

fn main() -> () {
    // creating jobs
    let j1 = Arc::new(Job::Spacer(1u64));
    let j2 = Arc::new(Job::Lambda(Box::new(async {
        println!("hello, world!")
    })));

    // creating a routine_builder
    let mut routine_builder = RoutineBuilder::new_with_capacity(2usize);
    let vec = routine_builder.as_mut();
    
    // adding jobs to the routine_builder
    vec.push(j1);
    vec.push(j2);

    // consuming and transforming routine_builder into routine
    let routine = routine_builder.into();

    // building a component
    let mut component_builder = ComponentBuilder::new().unwrap();
    component_builder.set_name("custom");
    component_builder.set_routine(routine);
    component_builder.set_handler(handler);

    #[allow(unused)]
    let component = component_builder
        .build()
        .unwrap();
}
