use anyhow::Result;
use crossbeam::channel::Receiver;

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
}

pub(super) struct Kernel {
    emg_data: f64,
    pause: Wait<bool>,
    pause_cache: bool,
    rx: Receiver<Message>,
}

impl Kernel {
    pub(super) fn new(pause: Wait<bool>, rx: Receiver<Message>) -> anyhow::Result<Self> {
        let pause_cache = pause.get()?;
        let kernel = Self {
            emg_data: 0.0,
            pause,
            pause_cache,
            rx,
        };
        Ok(kernel)
    }
}

impl Component for Kernel {
    fn run(mut self, _: &Config) -> Result<()> {
        self.rx.into_iter().for_each(|message| match message {
            Message::Bms(battery_report) => {
                let should_pause = match battery_report {
                    BatteryReport::High | BatteryReport::Medium => false,
                    BatteryReport::Low => true,
                };
                if self.pause_cache != should_pause {
                    self.pause.set(should_pause).ok();
                    self.pause_cache = should_pause;
                };
            },
            Message::Emg(data) => {
                println!("emg data: {}", data);
                self.emg_data = data;
            },
        });
        Ok(())
    }
}
