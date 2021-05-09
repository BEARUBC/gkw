/* external crates */

/* external uses */
use tokio::runtime::Builder;
use std::{
    thread,
    time::Duration,
    future::Future,
};
use async_framework::prelude::*;

/* internal mods */

/* internal uses */

async fn handler<F>(message: F) -> ()
where
F: Future,
F: Send + 'static, {
    message.await;
}

fn main() -> () {
    let basic = Component::new(String::from("basic"), handler);

    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                thread::sleep(Duration::from_secs(3u64));

                basic
                    .send(async {
                        println!("this is a raw future");
                    })
                    .unwrap();
            };
        });
}
