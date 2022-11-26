mod grip;
mod fsr;
mod emg;

#[cfg(feature = "tcp_edge")]
use std::thread::spawn;

use anyhow::Result;

#[cfg(feature = "tcp_edge")]
use crate::components::utils::create_tcp_runner;
use crate::components::Component;
#[cfg(feature = "tcp_edge")]
use crate::config::Components;
use crate::config::Config;
#[cfg(feature = "tcp_edge")]
use crate::config::TcpComponent;
use crate::wait::Wait;

pub(super) struct Kernel {
    pub(super) pause: Wait<bool>,
}

#[cfg(not(feature = "tcp_edge"))]
impl Component for Kernel {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_edge")]
impl Component for Kernel {
    fn run(
        self,
        Config {
            components: Components { emg, fsr, .. },
            ..
        }: &Config,
    ) -> Result<()> {
        {
            let TcpComponent { host, port } = emg;
            let mut state = emg::State::default();
            let parser = move |data| emg::parser(&mut state, data);
            let runner = create_tcp_runner(host, *port, parser, Some(self.pause.clone()))?;
            spawn(runner);
        };
        {
            let TcpComponent { host, port } = fsr;
            let mut state = fsr::State::default();
            let parser = move |data: f64| fsr::parser(&mut state, data);
            let runner = create_tcp_runner(host, *port, parser, Some(self.pause))?;
            spawn(runner);
        };
        Ok(())
    }
}
