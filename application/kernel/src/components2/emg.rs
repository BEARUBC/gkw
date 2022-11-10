use anyhow::Result;
use crossbeam::channel::Sender;

use crate::components2::kernel;
#[cfg(feature = "simulation")]
use crate::components2::utils::parser;
#[cfg(feature = "simulation")]
use crate::components2::utils::run_tcp;
use crate::components2::Component;
#[cfg(feature = "simulation")]
use crate::config;
#[cfg(feature = "simulation")]
use crate::config::Components;
use crate::config::Config;
use crate::wait::Wait;

pub(super) type Data = f64;

pub(super) struct Emg {
    pub(super) tx: Sender<kernel::Message>,
    pub(super) pause: Wait<bool>,
}

#[cfg(not(feature = "simulation"))]
impl Component for Emg {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "simulation")]
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
        let parser = parser(
            move |data| {
                let message = kernel::Message::Emg(data);
                self.tx.send(message)?;
                Ok(())
            },
            Some(self.pause),
        );
        run_tcp(host, *port, parser)?;
        Ok(())
    }
}
