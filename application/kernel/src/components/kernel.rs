use anyhow::Result;
use crossbeam::channel::Receiver;

use crate::components::bms;
use crate::components::bms::BatteryReport;
use crate::components::emg;
use crate::components::Component;
use crate::config::Config;
use crate::wait::Wait;

pub(super) enum Message {
    Emg(emg::Data),
    Bms(bms::BatteryReport),
}

pub(super) struct Kernel {
    pub(super) emg_data: f64,
    pub(super) pause: Wait<bool>,
    pub(super) rx: Receiver<Message>,
}

impl Component for Kernel {
    fn run(mut self, _: &Config) -> Result<()> {
        self.rx.into_iter().for_each(|message| match message {
            Message::Bms(battery_report) => {
                let should_pause = match battery_report {
                    BatteryReport::High | BatteryReport::Medium => false,
                    BatteryReport::Low => true,
                };
                self.pause.set(should_pause).ok();
            },
            Message::Emg(data) => {
                println!("emg data: {}", data);
                self.emg_data = data;
            },
        });
        Ok(())
    }
}
