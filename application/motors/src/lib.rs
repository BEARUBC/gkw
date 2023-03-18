use std::convert::TryInto;
use std::thread;
use std::thread::JoinHandle;
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
use std::cmp::min;
mod grip;


pub struct Motor {
    maestro: Maestro,
    targetPosition: Arc<Mutex<f64>>,
    increment: u16,
    channels: [Channel; 3usize]
}


impl Motor {
    pub fn new(targetArc: Arc<Mutex<f64>>, incrementVal: u16) -> Result<Motor, StdError> {
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


    pub fn step(&mut self) -> Result<(), StdError>{
        let grip: Grip = (self.targetPosition).lock().unwrap();
        let targets: [u16; 3usize] = grip.clone().into();

        for i in 0..3 {
            let mut channel = self.channels[i];
            let currentPosition = maestro.get_position(channel).unwrap();
            let targetPosition = targets[i];
            let nextPosition = min(targetPosition, 
                currentPosition +((targetPosition - currentPosition) as f64).signum() * self.increment
            );
            self.maestro.set_target(channel, nextPosition).unwrap();
        }

        Ok(())
    }

    pub fn run(&mut self) -> () {
        let mut thread: JoinHandle<Result<(), StdError>> = thread::spawn(move || {
            loop {
                self.step().unwrap();
            }

            Err(StdError::new(ErrorKind::Other, "MOTOR ERROR"));
        });
    }
}