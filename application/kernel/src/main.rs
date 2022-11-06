mod components;
mod config;
mod wait;

fn main() -> ! {
    let config = config::init().expect("Failed to initialize configurations.");
    components::run(config).expect("Failed to run components.");
    unreachable!("Application shut down unexpectedly.");
}
