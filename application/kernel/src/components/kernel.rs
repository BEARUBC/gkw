use anyhow::Result;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::components::analytics;
use crate::components::bms;
use crate::components::bms::BatteryReport;
use crate::components::emg;
use crate::components::Component;
use crate::config::Config;
use crate::wait::Wait;

pub(super) const MESSAGE_CAPACITY: usize = 16;

pub(super) enum Message {
    Emg(emg::Data),
    Bms(bms::BatteryReport),
    Analytics(analytics::GripType),
}

pub(super) struct Kernel {
    pub(super) pause: Wait<bool>,
    pub(super) analytics_tx: Sender<analytics::Data>,
    pub(super) rx: Receiver<Message>,
}

impl Component for Kernel {
    fn run(mut self, _: &Config) -> Result<()> {
        let mut pause_cache = self.pause.get()?;
        let mut grip_type_cache = analytics::GripType::default();
        self.rx
            .into_iter()
            .filter_map(move |message| {
                match message {
                    Message::Bms(battery_report) => {
                        let should_pause = match battery_report {
                            BatteryReport::High | BatteryReport::Medium => false,
                            BatteryReport::Low => true,
                        };
                        if pause_cache != should_pause {
                            self.pause.set(should_pause).ok()?;
                            pause_cache = should_pause;
                        };
                    },
                    Message::Emg(data) => {
                        let message = analytics::Data { emg: data };
                        self.analytics_tx.send(message).ok()?;
                    },
                    Message::Analytics(grip_type) => {
                        if grip_type_cache != grip_type {
                            #[cfg(not(release))]
                            println!("grip type: {:#?}", grip_type);
                            grip_type_cache = grip_type;
                        }
                    },
                };
                Some(())
            })
            .for_each(|()| ());
        Ok(())
    }
}
