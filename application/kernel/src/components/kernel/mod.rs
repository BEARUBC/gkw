mod emg;
mod fsr;
mod grip;

#[cfg(feature = "tcp_edge")]
use std::thread::spawn;
#[cfg(target_arch = "arm")]
use std::time::Duration;

use anyhow::Result;
#[cfg(target_arch = "arm")]
use raestro::maestro::builder::Builder;
use raestro::maestro::constants::Baudrate;
#[cfg(target_arch = "arm")]
use raestro::maestro::Maestro;

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
        let mut maestro: Maestro = Builder::default()
            .baudrate(Baudrate::Baudrate11520)
            .block_duration(Duration::from_micros(100))
            .try_into()?;
        let handler = move |data| emg::handler(&mut maestro, &mut state, data);
        let runner = create_tcp_runner(host, *port, handler, Some(pause))?;
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
