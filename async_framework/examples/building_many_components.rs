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
    // COMPONENT 0
    // creating jobs
    let j0 = Arc::new(Job::Spacer(1u64));
    let j1 = Arc::new(Job::Lambda(Box::new(async {
        println!("hello, world!")
    })));

    // creating a routine_builder
    let mut routine_builder0 = RoutineBuilder::new_with_capacity(2usize);
    let vec0 = routine_builder0.as_mut();
    
    // adding jobs to the routine_builder
    vec0.push(j0);
    vec0.push(j1);

    // consuming and transforming routine_builder into routine
    let routine0 = routine_builder0
        .build()
        .unwrap();

    // building a component
    let mut component_builder0 = ComponentBuilder::new().unwrap();
    component_builder0.set_name("custom");
    component_builder0.set_routine(routine0);
    component_builder0.set_handler(handler);

    #[allow(unused)]
    let component0 = component_builder0
        .build()
        .unwrap();
    
    // COMPONENT 1
    // creating jobs
    let j2 = Arc::new(Job::Spacer(1u64));
    let j3 = Arc::new(Job::Lambda(Box::new(async {
        println!("hello, world!")
    })));

    // creating a routine_builder
    let mut routine_builder1 = RoutineBuilder::new_with_capacity(2usize);
    let vec1 = routine_builder1.as_mut();
    
    // adding jobs to the routine_builder
    vec1.push(j2);
    vec1.push(j3);

    // consuming and transforming routine_builder into routine
    let routine = routine_builder1
        .build()
        .unwrap();

    // building a component
    let mut component_builder1 = ComponentBuilder::new().unwrap();
    component_builder1.set_name("custom");
    component_builder1.set_routine(routine);
    component_builder1.set_handler(handler);

    #[allow(unused)]
    let component1 = component_builder1
        .build()
        .unwrap();
}
