use std::convert::TryInto;
use std::thread;
use std::time::Duration;
use std::io::Error as StdError;
use raestro::maestro::builder::Builder;
use raestro::maestro::constants::Baudrate;
use raestro::maestro::constants::Channel;
use raestro::maestro::constants::MAX_QTR_PWM;
use raestro::maestro::constants::MIN_QTR_PWM;
use raestro::maestro::Maestro;
use std::sync::atomic::AtomicBool;
use std::collections::Vec;
mod grip;



pub struct Motor {
    maestro: Maestro,
    targetPosition: Arc<Mutex<u16>>,
    increment: u16,
    channels: [Channel; 3usize]
};


impl Motor {
    pub fn new(targetArc: Arc<Mutex<u16>>, incrementVal: u16) -> Result<Motor, StdError> {
        let mut maestro: Maestro = Maestro = Builder::default()
            .baudrate(Baudrate::Baudrate11520)
            .block_duration(Duration::from_millis(100))
            .try_into()
            .expect("Failed to build a `maestro` instance.");
        
        let channels = [Channel::Channel0, Channel::Channel1, Channel::Channel2];

        Ok(
            Motor {
                maestro: maestro,
                targetPosition: targetArc,
                increment: incrementVal,
                channels: channels
            }
        )
    }
}