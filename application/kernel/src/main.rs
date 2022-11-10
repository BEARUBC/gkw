mod components2;
mod config;
mod wait;

use crate::components2::run;
use crate::config::init;

fn main() -> ! {
    let config = init().expect("Failed to initialize configurations.");
    run(config).expect("Failed to run components.");
    unreachable!("Application shut down unexpectedly.");
}
