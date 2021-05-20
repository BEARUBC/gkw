use tokio::runtime::Builder;
use std::{
    thread,
    time::Duration,
    future::Future,
    sync::Arc,
};

use async_framework::{component::component::Component, job::Job, routine::builder::RoutineBuilder};

async fn handler<F>(message: F) -> ()
where
F: Future,
F: Send + 'static, {
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

    use std::{
        pin::Pin,
        task::{
            Context,
            Poll,
        },
    };

    struct MessageSystem;
    impl Future for MessageSystem {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            println!("Async poll");
            Poll::Ready(())
        }
    }
    async fn handler(_: MessageSystem) {
        println!("async handler runnin', bitch!")
    }

    let mut custom = Component::<MessageSystem>::new("custom").unwrap();

    custom.start(routine, handler).unwrap();

    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3u64));
                custom.send(MessageSystem).unwrap();
                custom.send(MessageSystem).unwrap();
            }
        });
}
