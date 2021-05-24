use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{
        Context,
        Poll
    }
};

use tokio::runtime::Builder as TokioBuilder;

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

async fn handler(message: MS) -> u32 {
    message.await;
    0u32
}

async fn lambda_1() -> u32 { println!("Hello, world, from lambda!"); 0u32 }

/* above is equivalent to:
fn lambda_1() -> F
where
F: 
 */

fn main() -> () {
    use std::env;

    let key = "RUST_BACKTRACE";
    env::set_var(key, "1");
    // assert_eq!(env::var(key), Ok("VALUE".to_string()));s

    // creating jobs
    let j1 = Arc::new(Job::Spacer(1u64));
    // let j2 = Arc::new(Job::Spacer(1u64));
    let j2 = Arc::new(Job::Lambda(Box::new(lambda_1)));

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
                std::thread::sleep(std::time::Duration::from_secs(3u64));
                component.send(MS).unwrap();
                component.send_run_request().unwrap();
            }
        });
}
