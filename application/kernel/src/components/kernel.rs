use anyhow::Result;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
#[cfg(feature = "simulation")]
use log::error;
#[cfg(feature = "simulation")]
use log::info;
#[cfg(feature = "simulation")]
use log::warn;

use crate::components::bms::BatteryReport;
use crate::components::emg::VoltageReading;
use crate::components::Component;
#[cfg(feature = "simulation")]
use crate::components::RESPONSE_CAPACITY;
use crate::config::Config;
use crate::wait::Wait;

pub(super) struct Kernel {
    pub(super) emg: Sender<Response>,
    pub(super) rx: Receiver<Message>,
}

#[derive(Default)]
struct InternalState {
    low_battery: Wait<bool>,
    last_message_sent: LastMessage,
}

enum LastMessage {
    Cont,
    Wait,
}

impl Default for LastMessage {
    fn default() -> Self {
        Self::Cont
    }
}

#[cfg(feature = "simulation")]
impl Kernel {
    fn probe_buffer(&self) -> bool {
        let len = self.emg.len();
        match len {
            _ if (10..=15).contains(&len) => {
                warn!("length: {}", len);
                false
            },
            _ if len == RESPONSE_CAPACITY => {
                error!("length: {}", len);
                true
            },
            _ => {
                info!("length: {}", len);
                false
            },
        }
    }

    fn handle(&self, message: Message, internal_state: &mut InternalState) -> Result<()> {
        let is_full = self.probe_buffer();
        match message {
            Message::Emg(voltage_reading) => {
                info!("Voltage Reading: {}", voltage_reading);
                if !is_full {
                    self.emg.send(Response::Continue)?;
                };
            },
            Message::Bms(battery_report) => match battery_report {
                BatteryReport::High | BatteryReport::Medium => {
                    internal_state.low_battery.set(false)?;
                    if let LastMessage::Wait = internal_state.last_message_sent {
                        if !is_full {
                            self.emg.send(Response::Continue)?;
                            internal_state.last_message_sent = LastMessage::Cont;
                        };
                    };
                },
                BatteryReport::Low => {
                    internal_state.low_battery.set(true)?;
                    if let LastMessage::Cont = internal_state.last_message_sent {
                        if !is_full {
                            let wait = internal_state.low_battery.clone();
                            let response = Response::Wait(wait);
                            self.emg.send(response)?;
                            internal_state.last_message_sent = LastMessage::Wait;
                        };
                    };
                },
            },
        };
        Ok(())
    }
}

#[cfg(not(feature = "simulation"))]
impl Component for Kernel {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}

#[cfg(feature = "simulation")]
impl Component for Kernel {
    fn run(self, _: &Config) -> Result<()> {
        let mut internal_state = InternalState::default();
        self.rx.iter().for_each(|message| {
            self.handle(message, &mut internal_state).ok();
        });
        Ok(())
    }
}

pub(super) enum Message {
    Bms(BatteryReport),
    Emg(VoltageReading),
}

pub(super) enum Response {
    Continue,
    Wait(Wait<bool>),
}

impl From<Response> for Option<Wait<bool>> {
    fn from(response: Response) -> Self {
        match response {
            Response::Continue => None,
            Response::Wait(wait) => Some(wait),
        }
    }
}
