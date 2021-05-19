use tokio::runtime::Builder;
use std::{
    thread,
    time::Duration,
    future::Future,
    sync::Arc,
};

use async_framework::{component::component::Component, job::Job, routine::routine_builder::RoutineBuilder};

async fn handler<F>(message: F) -> ()
where
F: Future,
F: Send + 'static, {
    message.await;
}

fn main() -> () {
    let j1 = Arc::new(Job::Spacer(1u64));
    let j2 = Arc::new(Job::Lambda(Box::new(async {})));

    let mut routine_builder = RoutineBuilder::new_with_capacity(2usize);
    let vec = routine_builder.as_mut();
    
    vec.push(j1);
    vec.push(j2);

    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {});
}
