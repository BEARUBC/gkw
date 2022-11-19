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

#[cfg_attr(not(release), derive(Debug))]
#[derive(PartialEq, Eq)]
pub(super) enum GripType {
    Hammer,
    Cup,
    Flat,
}

impl Default for GripType {
    fn default() -> Self {
        Self::Flat
    }
}

#[cfg(feature = "pseudo_analytics")]
impl From<f64> for GripType {
    fn from(data: f64) -> Self {
        const MODULO_BASE: u64 = 3;
        let data = data.floor() as u64;
        let data = data % MODULO_BASE;
        match data {
            0 => Self::Hammer,
            1 => Self::Cup,
            _ => Self::Flat,
        }
    }
}

pub(super) struct Analytics {
    pub(super) pause: Wait<bool>,
}

#[cfg(not(feature = "tcp_edge"))]
impl Component for Analytics {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_edge")]
impl Component for Analytics {
    fn run(
        self,
        Config {
            components: Components { emg, fsr, .. },
            ..
        }: &Config,
    ) -> Result<()> {
        {
            let TcpComponent { host, port } = emg;
            let mut grip_type_cache = GripType::default();
            #[cfg(feature = "pseudo_analytics")]
            let parser = move |data: f64| {
                let grip_type = GripType::from(data);
                if grip_type_cache != grip_type {
                    #[cfg(not(release))]
                    println!("grip type: {:#?}", grip_type);
                    grip_type_cache = grip_type;
                };
                Ok(())
            };
            #[cfg(not(feature = "pseudo_analytics"))]
            let parser = |_: f64| Ok(());
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
