#[cfg(feature = "pseudo_analytics")]
use std::ops::RangeInclusive;
#[cfg(feature = "pseudo_analytics")]
use std::thread::spawn;

use anyhow::Result;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::components::emg;
use crate::components::kernel::Message;
use crate::components::Component;
use crate::config::Config;

pub(super) const MESSAGE_CAPACITY: usize = 256;

#[cfg(feature = "pseudo_analytics")]
const HAMMER_DATA_RANGE: RangeInclusive<f64> = 0.0..=10.0;

#[cfg(feature = "pseudo_analytics")]
const CUP_DATA_RANGE: RangeInclusive<f64> = 10.0..=20.0;

pub(super) struct Data {
    pub(super) emg: emg::Data,
}

#[cfg_attr(not(release), derive(Debug))]
#[derive(PartialEq, Eq)]
pub(super) enum GripType {
    Hammer,
    Cup,
    Flat,
}

#[cfg(feature = "pseudo_analytics")]
impl Default for GripType {
    fn default() -> Self {
        Self::Flat
    }
}

#[cfg(feature = "pseudo_analytics")]
impl From<Data> for GripType {
    fn from(Data { emg }: Data) -> Self {
        match emg {
            _ if HAMMER_DATA_RANGE.contains(&emg) => Self::Hammer,
            _ if CUP_DATA_RANGE.contains(&emg) => Self::Cup,
            _ => Self::default(),
        }
    }
}

pub(super) struct Analytics {
    pub(super) tx: Sender<Message>,
    pub(super) rx: Receiver<Data>,
}

#[cfg(feature = "pseudo_analytics")]
impl Component for Analytics {
    fn run(self, _: &Config) -> Result<()> {
        spawn(move || {
            self.rx
                .into_iter()
                .filter_map(|data| {
                    let grip_type = data.into();
                    let message = Message::Analytics(grip_type);
                    self.tx.send(message).ok()?;
                    Some(())
                })
                .for_each(|()| ());
        });
        Ok(())
    }
}

#[cfg(not(feature = "pseudo_analytics"))]
impl Component for Analytics {
    fn run(self, _: &Config) -> Result<()> {
        todo!()
    }
}
