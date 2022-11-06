#[cfg(feature = "simulation")]
use std::ops::Range;
use std::ops::RangeInclusive;

use anyhow::Result;
use crossbeam::channel::Sender;

#[cfg(feature = "simulation")]
use crate::components::kernel;
use crate::components::kernel::Message;
#[cfg(feature = "simulation")]
use crate::components::utils;
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

const MAX_BATTERY: f64 = 100.0;
const HIGH_BATTERY_RANGE_CUTOFF: f64 = 70.0;
const MEDIUM_BATTERY_RANGE_CUTOFF: f64 = 20.0;
const HIGH_BATTERY_RANGE: RangeInclusive<f64> = HIGH_BATTERY_RANGE_CUTOFF..=MAX_BATTERY;
const MEDIUM_BATTERY_RANGE: RangeInclusive<f64> =
    MEDIUM_BATTERY_RANGE_CUTOFF..=HIGH_BATTERY_RANGE_CUTOFF;

pub(super) enum BatteryReport {
    High,
    Medium,
    Low,
}

impl From<f64> for BatteryReport {
    fn from(battery_percentage: f64) -> Self {
        if HIGH_BATTERY_RANGE.contains(&battery_percentage) {
            BatteryReport::High
        } else if MEDIUM_BATTERY_RANGE.contains(&battery_percentage) {
            BatteryReport::Medium
        } else {
            BatteryReport::Low
        }
    }
}

pub(super) struct Bms {
    pub(super) tx: Sender<Message>,
}

#[cfg(feature = "simulation")]
impl Component for Bms {
    fn run(self, config: &Config) -> Result<()> {
        self.run_tcp(config)
    }
}

#[cfg(not(feature = "simulation"))]
impl Component for Bms {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "simulation")]
impl ForwardingComponent for Bms {
    type Message = Message;

    const DESTINATION_BUFFER_CAPACITY: usize = kernel::MESSAGE_CAPACITY;
    const DESTINATION_BUFFER_CAPACITY_WARNING_INTERVAL: Range<usize> =
        kernel::MESSAGE_CAPACITY_WARNING_INTERVAL;

    fn tx(&self) -> &Sender<Self::Message> {
        &self.tx
    }
}

#[cfg(feature = "simulation")]
impl TcpComponent for Bms {
    fn tcp_config(
        Config {
            components:
                Components {
                    bms: config::Bms { host, port },
                    ..
                },
        }: &Config,
    ) -> (&str, &u16) {
        (host, port)
    }

    fn handle(&self, buffer: &[u8]) -> Result<()> {
        let message = utils::parse_float(buffer)?;
        let battery_report = message.into();
        let message = Message::Bms(battery_report);
        self.send(message)?;
        Ok(())
    }
}
