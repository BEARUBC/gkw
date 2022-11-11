#[cfg(feature = "pseudo_analytics")]
use std::ops::RangeInclusive;
use std::thread::spawn;

use anyhow::Result;

use crate::components::utils::run_tcp;
use crate::components::Component;
use crate::config;
use crate::config::Config;
use crate::config::TcpEdge;
use crate::wait::Wait;

#[cfg(feature = "pseudo_analytics")]
const HAMMER_DATA_RANGE: RangeInclusive<f64> = 0.0..=10.0;

#[cfg(feature = "pseudo_analytics")]
const CUP_DATA_RANGE: RangeInclusive<f64> = 10.0..=20.0;

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
        match data {
            _ if HAMMER_DATA_RANGE.contains(&data) => Self::Hammer,
            _ if CUP_DATA_RANGE.contains(&data) => Self::Cup,
            _ => Self::default(),
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
            tcp_edge:
                TcpEdge {
                    emg: config::Emg { host, port },
                    ..
                },
            ..
        }: &Config,
    ) -> Result<()> {
        #[cfg(feature = "pseudo_analytics")]
        let parser = move |data: f64| {
            let grip_type = GripType::from(data);
            println!("grip type: {:#?}", grip_type);
            Ok(())
        };
        #[cfg(not(feature = "pseudo_analytics"))]
        let parser = move |_: f64| Ok(());
        let runner = run_tcp(host, *port, parser, Some(self.pause))?;
        spawn(runner);
        Ok(())
    }
}
