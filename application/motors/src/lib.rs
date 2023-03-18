use std::convert::TryInto;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::io::Error as StdError;
use raestro::maestro::builder::Builder;
use raestro::maestro::constants::Baudrate;
use raestro::maestro::constants::Channel;
use raestro::maestro::Maestro;
use std::sync::{Arc, Mutex};
use std::cmp::min;
mod grip;
use grip::*;


pub struct Motor {
    //maestro: Maestro,
    targetPosition: Arc<Mutex<f64>>,
    increment: u16,
    channels: [Channel; 3usize],
    runThread: JoinHandle<()>
}


impl Motor {
    pub fn new(incrementVal: u16) -> Result<Motor, StdError> {
        let mut maestro: Maestro = Builder::default()
            .baudrate(Baudrate::Baudrate11520)
            .block_duration(Duration::from_millis(100))
            .try_into()
            .expect("Failed to build a `maestro` instance.");
        
        let channels = [Channel::Channel0, Channel::Channel1, Channel::Channel2];

        let targetArc: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.5 as f64));

        let targetClone = (targetArc).clone();

        Ok(
            Motor {
                targetPosition: targetArc,
                increment: incrementVal,
                channels: channels,
                runThread: thread::spawn(move || {
                    loop {
                        let grip: Grip = (*(targetClone.lock().unwrap())).into();
                        let targets: [u16; 3usize] = grip.clone().into();

                        for i in 0..3 {
                            let channel = channels[i];
                            let currentPosition = maestro.get_position(channel).unwrap();
                            let targetPosition = targets[i];
                            let nextPosition = min(targetPosition, 
                                currentPosition + (((targetPosition - currentPosition) as f64).signum() as u16) * (incrementVal)
                            );
                            maestro.set_target(channel, nextPosition).unwrap();
                        }
                    }
                })
            }
        )
    }


    pub fn cahngeGrip(&mut self, gripVal: f64) {
        let mut targetGuard = self.targetPosition.lock().unwrap();

        *targetGuard = gripVal;
    }

    // pub fn step(&mut self) -> Result<(), StdError>{
    //     let targetClone = (self.targetPosition).clone();
    //     let grip: Grip = (*(targetClone.lock().unwrap())).into();
    //     let targets: [u16; 3usize] = grip.clone().into();

    //     for i in 0..3 {
    //         let mut channel = self.channels[i];
    //         let currentPosition = self.maestro.get_position(channel).unwrap();
    //         let targetPosition = targets[i];
    //         let nextPosition = min(targetPosition, 
    //             currentPosition + (((targetPosition - currentPosition) as f64).signum() as u16) * (self.increment)
    //         );
    //         self.maestro.set_target(channel, nextPosition).unwrap();
    //     }

    //     Ok(())
    // }

    // pub fn run(&mut self) -> () {
    //     let mut thread: JoinHandle<()> = thread::spawn(move || {
    //         loop {
    //             match self.step() {
    //                 Ok(()) => (),
    //                 Err(err) => break,
    //             }
    //         }
    //     });
    // }
}