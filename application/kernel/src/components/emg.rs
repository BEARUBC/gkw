use anyhow::Result;
use crossbeam::channel::Sender;

use crate::components::kernel;
#[cfg(feature = "tcp_data")]
use crate::components::utils::run_tcp;
use crate::components::Component;
#[cfg(feature = "tcp_data")]
use crate::config;
#[cfg(feature = "tcp_data")]
use crate::config::Components;
use crate::config::Config;
use crate::wait::Wait;

pub(super) type Data = f64;

pub(super) struct Emg {
    tx: Sender<kernel::Message>,
    pause: Wait<bool>,
}

impl Emg {
    pub(super) fn new(tx: Sender<kernel::Message>, pause: Wait<bool>) -> Self {
        Self { tx, pause }
    }
}

#[cfg(not(feature = "tcp_data"))]
impl Component for Emg {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_data")]
impl Component for Emg {
    fn run(
        self,
        Config {
            components:
                Components {
                    emg: config::Emg { host, port },
                    ..
                },
            ..
        }: &Config,
    ) -> Result<()> {
        run_tcp(
            host,
            *port,
            move |data| {
                let message = kernel::Message::Emg(data);
                self.tx.send(message)?;
                Ok(())
            },
            Some(self.pause),
        )?;
        Ok(())
    }
}
