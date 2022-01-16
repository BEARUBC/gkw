/* external crates */

/* external uses */
use tokio::runtime::Builder;
use std::{
    thread,
    time::Duration,
};
use async_framework::prelude::*;

/* internal mods */
mod asynchronous {
    use std::{
        future::Future,
        task::{
            Poll,
            Context,
        },
        pin::Pin,
    };
    use super::embedded_data::EmbeddedData;

    #[derive(Debug)]
    pub enum Asynchronous {
        Variant1,
        Variant2(EmbeddedData),
    }

    impl Future for Asynchronous {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
            println!("Async poll");

            match self.into_ref().get_ref() {
                Self::Variant1 => (),
                Self::Variant2(_) => (),
            }
            return Poll::Pending;
        }
    }
}
mod embedded_data {
    #[derive(Debug)]
    pub struct EmbeddedData;
}

/* internal uses */
use asynchronous::Asynchronous;
use embedded_data::EmbeddedData;

async fn handler(message: Asynchronous) -> () {
    match message {
        Asynchronous::Variant1 => {
            println!("doing some heavy computations with this variant!");
            // ... heavy computations here
        },
        Asynchronous::Variant2(_) => println!("ignoring this message!"),
    };
}

fn main() -> () {
    let custom = Component::<Asynchronous>::new(String::from("custom"), handler);

    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            loop {
                thread::sleep(Duration::from_secs(3u64));

                custom
                    .send(Asynchronous::Variant1)
                    .unwrap();
                custom
                    .send(Asynchronous::Variant2(EmbeddedData))
                    .unwrap();
            };
        });
}
