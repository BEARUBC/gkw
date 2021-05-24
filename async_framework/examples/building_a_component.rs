use std::{future::Future, pin::Pin, task::{
        Context,
        Poll
    }};

use tokio::runtime::Builder as TokioBuilder;

use async_framework::{
    component_builder::builder::ComponentBuilder,
    job::Job,
    routine_builder::builder::RoutineBuilder,
    contacts::contacts::Contacts,
    builder::Builder,
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
    tokio::time::sleep(std::time::Duration::from_secs(10u64)).await;

    let result = message.await;
    println!("result: {}", result);

    result
}

async fn lambda_1(_: Contacts<MS>) -> u32 {
    println!("Hello, world, from lambda!");
    0u32
}

/* above is equivalent to:
fn lambda_1() -> F
where
F: 
*/

fn main() -> () {
    use std::env;

    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");

    // creating jobs
    let j1 = Job::from_spacer(2u64);
    let j2 = Job::from_lambda(lambda_1);

    // creating a routine_builder
    let mut routine_builder = RoutineBuilder::with_capacity(2usize);
    
    // adding jobs to the routine_builder
    routine_builder.push(j1);
    routine_builder.push(j2);

    // consuming and transforming routine_builder into routine
    let routine = routine_builder.build().unwrap();

    // building a component
    let component_builder = ComponentBuilder::new(
        "custom",
        routine,
        handler,
    ).unwrap();

    let mut component = component_builder
        .build()
        .unwrap();
    
    component.start().unwrap();

    TokioBuilder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1u64));
                println!("sending message");
                component.send(MS).unwrap();
                // component.run_next_job().unwrap();
            }
        });
}
