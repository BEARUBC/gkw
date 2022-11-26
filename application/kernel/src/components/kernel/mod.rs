mod grip;
mod parser;

#[cfg(feature = "tcp_edge")]
use std::thread::spawn;

use anyhow::Result;

use crate::components::kernel::grip::Grip;
#[cfg(feature = "tcp_edge")]
use crate::components::utils::create_tcp_runner;
use crate::components::Component;
#[cfg(feature = "tcp_edge")]
use crate::config::Components;
use crate::config::Config;
#[cfg(feature = "tcp_edge")]
use crate::config::TcpComponent;
use crate::wait::Wait;

#[derive(Default)]
struct State {
    grip: Grip,
}

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
        let mut state = State::default();
        {
            let TcpComponent { host, port } = emg;
            let parser = move |data| parser::parser(&mut state, data);
            let runner = create_tcp_runner(host, *port, parser, Some(self.pause.clone()))?;
            spawn(runner);
        };
        {
            let TcpComponent { host, port } = fsr;
            let parser = |_: f64| Ok(());
            let runner = create_tcp_runner(host, *port, parser, Some(self.pause))?;
            spawn(runner);
        };
        Ok(())
    }
}
