use anyhow::Result;
use crossbeam::channel::Sender;

use crate::components2::kernel;
#[cfg(feature = "simulation")]
use crate::components2::utils::parser;
#[cfg(feature = "simulation")]
use crate::components2::utils::run_tcp;
use crate::components2::Component;
#[cfg(feature = "simulation")]
use crate::config;
#[cfg(feature = "simulation")]
use crate::config::Components;
use crate::config::Config;

pub(super) enum BatteryReport {
    High,
    Medium,
    Low,
}

pub(super) struct Bms {
    pub(super) tx: Sender<kernel::Message>,
}

#[cfg(not(feature = "simulation"))]
impl Component for Bms {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "simulation")]
impl Component for Bms {
    fn run(
        self,
        Config {
            components:
                Components {
                    bms:
                        config::Bms {
                            host,
                            port,
                            high_battery_cutoff,
                            medium_battery_cutoff,
                        },
                    ..
                },
            ..
        }: &Config,
    ) -> Result<()> {
        let high_battery_cutoff = *high_battery_cutoff;
        let medium_battery_cutoff = *medium_battery_cutoff;
        let parser = parser(
            move |data| {
                let battery_report = match data {
                    _ if (high_battery_cutoff..=100.0).contains(&data) => BatteryReport::High,
                    _ if (medium_battery_cutoff..=high_battery_cutoff).contains(&data) => {
                        BatteryReport::Medium
                    },
                    _ => BatteryReport::Low,
                };
                let message = kernel::Message::Bms(battery_report);
                self.tx.send(message)?;
                Ok(())
            },
            None,
        );
        run_tcp(host, *port, parser)?;
        Ok(())
    }
}
