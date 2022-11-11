use anyhow::Result;
use crossbeam::channel::Sender;

use crate::components::kernel;
#[cfg(feature = "tcp_edge")]
use crate::components::utils::run_tcp;
use crate::components::Component;
#[cfg(feature = "tcp_edge")]
use crate::config;
use crate::config::Config;
#[cfg(feature = "tcp_edge")]
use crate::config::TcpEdge;
use crate::wait::Wait;

pub(super) type Data = f64;

pub(super) struct Emg {
    pub(super) tx: Sender<kernel::Message>,
    pub(super) pause: Wait<bool>,
}

#[cfg(not(feature = "tcp_edge"))]
impl Component for Emg {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_edge")]
impl Component for Emg {
    fn run(
        self,
        Config {
            tcp_edge:
                TcpEdge {
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
