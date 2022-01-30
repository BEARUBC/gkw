use std::io;
use std::process::exit;

use log::error;
use log::info;

mod error;
mod kernel;

/// Initialize logging.
///
/// For `not(release)` builds, `log-level=trace` will be executed.
/// For `release` builds, `log-level=info` will be executed.
///
/// To learn more about logging and the various levels, visit: https://docs.rs/env_logger/latest/env_logger/.
///
/// In short, the logging precedence is as follows:
/// 1. `error`
/// 2. `warn`
/// 3. `info`
/// 4. `debug`
/// 5. `trace`
fn init_logging() {
    #[cfg(not(release))]
    std::env::set_var("RUST_LOG", "trace");
    #[cfg(release)]
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();
}

/// Initialization sequence.
///
/// Serves to --apart from other things-- initialize instances of lazy-statics across GKW.
async fn init() -> io::Result<()> {
    info!("Initializing GKW...");

    // ... initialization sequences go here

    info!("GKW initialization done.");

    Ok(())
}

/// Main entrypoint into the GKW software.
///
/// The default number of threads on our Raspberry Pi is 4.
/// Therefore, without hyper-threading the CPU, the number of native-threads that should be
/// instantiated is 4.
#[allow(unreachable_code)]
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> ! {
    init_logging();

    match init().await {
        Ok(()) => info!("GKW initialization has succeeded."),
        Err(err) => {
            error!("GKW initialization has failed with error: {}.", err);
            exit(1);
        },
    };

    #[allow(clippy::empty_loop)]
    loop {}

    error!("The GKW main-loop has crashed unexpectedly. Exiting...");
    unreachable!()
}
