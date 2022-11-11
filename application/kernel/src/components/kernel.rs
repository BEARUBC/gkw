use anyhow::Result;

use crate::components::utils::run_tcp;
use crate::components::Component;
use crate::config;
use crate::config::Config;
use crate::config::TcpEdge;
use crate::wait::Wait;

const HIGH_BATTERY_CUTOFF: f64 = 70.0;
const MEDIUM_BATTERY_CUTOFF: f64 = 20.0;

pub(super) struct Kernel {
    pub(super) pause: Wait<bool>,
}

#[derive(PartialEq, Eq)]
enum BatteryState {
    High,
    Medium,
    Low,
}

impl Default for BatteryState {
    fn default() -> Self {
        Self::High
    }
}

impl From<f64> for BatteryState {
    fn from(battery_level: f64) -> Self {
        match battery_level {
            _ if battery_level >= HIGH_BATTERY_CUTOFF => Self::High,
            _ if battery_level >= MEDIUM_BATTERY_CUTOFF => Self::Medium,
            _ => Self::Low,
        }
    }
}

impl BatteryState {
    fn should_pause(self) -> bool {
        match self {
            Self::High | Self::Medium => false,
            Self::Low => true,
        }
    }
}

#[cfg(not(feature = "tcp_edge"))]
impl Component for Kernel {
    fn run(mut self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "tcp_edge")]
impl Component for Kernel {
    fn run(
        mut self,
        Config {
            tcp_edge:
                TcpEdge {
                    bms: config::Bms { host, port },
                    ..
                },
            ..
        }: &Config,
    ) -> Result<()> {
        let mut should_pause_cache = self.pause.get()?;
        run_tcp(
            host,
            *port,
            move |battery_level: f64| {
                let should_pause = BatteryState::from(battery_level).should_pause();
                if should_pause_cache != should_pause {
                    self.pause.set(should_pause)?;
                    should_pause_cache = should_pause;
                };
                Ok(())
            },
            None,
        )?();
        Ok(())
    }
}
