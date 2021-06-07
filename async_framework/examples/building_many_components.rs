use std::{
    future::Future,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};

use async_framework::{
    builder::Builder,
    component_builder::builder::ComponentBuilder,
    contacts::contacts::Contacts,
    job::Job,
    routine_builder::builder::RoutineBuilder,
    system_builder::system_builder::SystemBuilder,
};

struct MS;

impl Future for MS {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        println!("handler for the MS");
        Poll::Ready(0u32)
    }
}

async fn handler(_: Contacts<MS>, message: MS) -> u32 {
    println!("handler called");
    tokio::time::sleep(std::time::Duration::from_secs(10u64)).await;

    let result = message.await;
    println!("result: {}", result);

    result
}

async fn lambda_1(_: Contacts<MS>) -> u32 {
    println!("1");
    0u32
}

async fn lambda_2(_: Contacts<MS>) -> u32 {
    println!("2");
    0u32
}

fn main() -> () {
    use std::env;

    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");

    // creating jobs
    let j1 = Job::from_spacer(5u64);
    let j2 = Job::from_lambda(lambda_1);
    let j3 = Job::from_spacer(2u64);
    let j4 = Job::from_lambda(lambda_2);

    // creating a routine_builder
    let mut routine_builder1 = RoutineBuilder::with_capacity(2usize);
    let mut routine_builder2 = RoutineBuilder::with_capacity(2usize);

    // adding jobs to the routine_builder
    routine_builder1.push(j1);
    routine_builder1.push(j2);
    routine_builder1.push(j3);
    routine_builder1.push(j4);

    // building a component
    let component_builder1 = ComponentBuilder::new(
        "custom1",
        routine_builder1,
        handler,
    ).unwrap();
    let component_builder2 = ComponentBuilder::new(
        "custom2",
        routine_builder2,
        handler,
    ).unwrap();

    let mut system_builder = SystemBuilder::with_capacity(2usize);

    let vec = system_builder.as_mut();
    vec.push(component_builder1);
    vec.push(component_builder2);

    let system = system_builder.build().unwrap();
    
    system.run()
}
