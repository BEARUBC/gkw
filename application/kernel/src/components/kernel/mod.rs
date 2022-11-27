mod emg;
mod fsr;
mod grip;

#[cfg(feature = "tcp_edge")]
use std::thread::spawn;

use anyhow::Result;
use raestro::Maestro;

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
        self.launch_emg(emg)?;
        self.launch_fsr(fsr)?;
        Ok(())
    }
}

impl Kernel {
    fn launch_emg(&self, TcpComponent { host, port }: &TcpComponent) -> Result<()> {
        let pause = self.pause.clone();
        let mut state = emg::State::default();
        let mut maestro = Maestro::new();

        let parser = move |data| emg::parser(&mut maestro, &mut state, data);
        let runner = create_tcp_runner(host, *port, parser, Some(pause))?;
        spawn(runner);
        Ok(())
    }

    fn launch_fsr(&self, TcpComponent { host, port }: &TcpComponent) -> Result<()> {
        let pause = self.pause.clone();
        let runner = create_tcp_runner(host, *port, fsr::parser, Some(pause))?;
        spawn(runner);
        Ok(())
    }
}
