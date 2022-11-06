use anyhow::Result;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::components::kernel::Message;
use crate::components::kernel::Response;
#[cfg(feature = "simulation")]
use crate::components::utils;
#[cfg(feature = "simulation")]
use crate::components::BackPressuredForwardingComponent;
use crate::components::Component;
#[cfg(feature = "simulation")]
use crate::components::ForwardingComponent;
#[cfg(feature = "simulation")]
use crate::components::TcpComponent;
#[cfg(feature = "simulation")]
use crate::config;
#[cfg(feature = "simulation")]
use crate::config::Components;
use crate::config::Config;

pub(super) type VoltageReading = f64;

pub(super) struct Emg {
    pub(super) tx: Sender<Message>,
    pub(super) rx: Receiver<Response>,
}

#[cfg(not(feature = "simulation"))]
impl Component for Emg {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "simulation")]
impl Component for Emg {
    fn run(self, config: &Config) -> Result<()> {
        self.run_tcp(config)
    }
}

#[cfg(feature = "simulation")]
impl ForwardingComponent for Emg {
    type Message = Message;

    fn tx(&self) -> &Sender<Self::Message> {
        &self.tx
    }
}

#[cfg(feature = "simulation")]
impl BackPressuredForwardingComponent for Emg {
    type Response = Response;
    type WaitCondition = bool;

    const WAIT_CONDITION: Self::WaitCondition = false;

    fn rx(&self) -> &Receiver<Response> {
        &self.rx
    }
}

#[cfg(feature = "simulation")]
impl TcpComponent for Emg {
    fn tcp_config(
        Config {
            components:
                Components {
                    emg: config::Emg { host, port },
                    ..
                },
        }: &Config,
    ) -> (&str, &u16) {
        (host, port)
    }

    fn handle(&self, buffer: &[u8]) -> Result<()> {
        let message = utils::parse_float(buffer)?;
        let message = Message::Emg(message);
        self.send_and_apply_pressure(message)
    }
}
