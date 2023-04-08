use raestro::maestro::builder::Builder;
use raestro::maestro::constants::Baudrate;
use raestro::maestro::constants::Channel;
use raestro::maestro::constants::MAX_QTR_PWM;
use raestro::maestro::constants::MIN_QTR_PWM;
use raestro::maestro::Maestro;
use std::cmp::{max, min};
use std::convert::TryInto;
use std::io::Error as StdError;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Motor {
    //maestro: Maestro,
    targetPosition: Arc<Mutex<f64>>,
    increment: u16,
    channels: [Channel; 1usize],
    runThread: JoinHandle<()>,
}

impl Motor {
    pub fn new(incrementVal: u16) -> Result<Motor, StdError> {
        let mut maestro: Maestro = Builder::default()
            .baudrate(Baudrate::Baudrate11520)
            .block_duration(Duration::from_millis(100))
            .try_into()
            .expect("Failed to build a `maestro` instance.");

        // let channels = [Channel::Channel0, Channel::Channel1, Channel::Channel2];
        let channels = [Channel::Channel0];

        let targetArc: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.5 as f64));

        let targetClone = (targetArc).clone();

        Ok(Motor {
            targetPosition: targetArc,
            increment: incrementVal,
            channels,
            runThread: thread::spawn(move || {
                let mut prev_position = [4000];
                loop {
                    let grip: Grip = (*(targetClone.lock().unwrap())).into();
                    // let targets: [u16; 3usize] = grip.clone().into();
                    let mut targets: [u16; 1usize] = grip.clone().into();

                    for i in 0..1 {
                        //0..3
                        let channel = channels[i];
                        println!("Channel {}", i);
                        let currentPosition = prev_position[i];
                        let targetPosition = targets[i];
                        println!("target Pos {}", targetPosition);
                        let mut nextPosition: u16;
                        if targetPosition > currentPosition {
                            nextPosition = currentPosition + incrementVal;
                            nextPosition = min(nextPosition, targetPosition);
                        } else {
                            nextPosition = currentPosition - incrementVal;
                            nextPosition = max(nextPosition, targetPosition);
                        }
                        //nextPosition = targetPosition;
                        prev_position[i] = nextPosition;
                        println!("Next Pos {}", nextPosition);
                        maestro.set_target(channel, nextPosition).unwrap();
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }),
        })
    }

    pub fn changeGrip(&mut self, gripVal: f64) {
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Grip {
    Hammer { x: f64 },
    Cup { x: f64 },
    Flat { x: f64 },
}

impl From<Grip> for [u16; 1usize] {
    fn from(grip: Grip) -> [u16; 1usize] {
        match grip {
            // Grip::Hammer{x} => [((4000 as f64)*x).floor() as u16, ((4000 as f64)*x).floor() as u16, ((4000 as f64)*x).floor() as u16],
            // Grip::Cup{x} => [((5000 as f64)*x).floor() as u16, ((5000 as f64)*x).floor() as u16, ((5000 as f64)*x).floor() as u16],
            // Grip::Flat{x} => [((6000 as f64)*x).floor() as u16, ((6000 as f64)*x).floor() as u16, ((6000 as f64)*x).floor() as u16],
            Grip::Hammer { x } => [((4000 as f64) * x).floor() as u16], // ((4000 as f64)*x).floor() as u16, ((4000 as f64)*x).floor() as u16],
            Grip::Cup { x } => [((5000 as f64) * x).floor() as u16], // ((5000 as f64)*x).floor() as u16, ((5000 as f64)*x).floor() as u16],
            Grip::Flat { x } => [((8000 as f64) * x).floor() as u16], // ((6000 as f64)*x).floor() as u16, ((6000 as f64)*x).floor() as u16],
        }
    }
}

impl Default for Grip {
    fn default() -> Self {
        Self::Flat { x: 1.0 as f64 }
    }
}

impl From<f64> for Grip {
    fn from(data: f64) -> Self {
        const MODULO_BASE: u16 = 3;
        let grip = (data.floor() as u16) % MODULO_BASE;
        let scale = 2.0 * (data - grip as f64);
        match grip {
            0 => Self::Hammer { x: scale },
            1 => Self::Cup { x: scale },
            _ => Self::Flat { x: scale },
        }
    }
}
