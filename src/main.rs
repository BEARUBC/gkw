use rust_gpiozero::*;
use gpio::{GpioIn, GpioOut};
use std::{thread, time};


fn main() {

    let mut motor = PWMOutputDevice::new(18); 

    motor.pulse(2.0, 3.0);

}



// fn main() {
//  // Let's open GPIO23 and -24, e.g. on a Raspberry Pi 2.
//     let mut motor = gpio::sysfs::SysFsGpioOutput::open(18).unwrap();
    
//     // GPIO18 will be toggled every second in the background by a different thread
//     let mut value = false;
//     thread::spawn(move || loop {
//         motor.set_value(value).expect("could not set gpio18");
//         thread::sleep(time::Duration::from_millis(1000));
//         value = !value;
//     });
    
//     // The main thread will simply display the current value of GPIO23 every 100ms.
//     loop {
//         println!("Motor: {:?}", motor.read_value().unwrap());
//         thread::sleep(time::Duration::from_millis(100));
//     }

// }

// use rppal::{pwm::Pwm, pwm::Channel, pwm::Polarity};

// use std::thread;
// use std::error::Error;
// use std::time::Duration;

// const PERIOD_MS: u64 = 200;
// const PULSE_MIN_US: u64 = 12000;
// const PULSE_NEUTRAL_US: u64 = 15000;
// const PULSE_MAX_US: u64 = 18000;

// fn main()-> Result<(), Box<dyn Error>>{

//     let pwm = Pwm::with_period(
//         Channel::Pwm0,
//         Duration::from_millis(PERIOD_MS),
//         Duration::from_micros(PULSE_MAX_US),
//         Polarity::Normal,
//         true
//     ).unwrap();
    


//     thread::sleep(Duration::from_millis(500));

//     pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US)).unwrap();

//     thread::sleep(Duration::from_millis(500));

//     pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US)).unwrap();

//     Ok(())

// }

