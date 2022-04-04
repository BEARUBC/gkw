use std::process::exit;

use log::error;
use log::info;

mod init;
mod kernel;

/// Main entrypoint into the GKW software.
///
/// The default number of threads on our Raspberry Pi is 4.
/// Therefore, without hyper-threading the CPU, the number of native-threads that should be
/// instantiated is 4.
#[allow(unreachable_code)]
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> ! {
    init::init_logging();

    match init::init().await {
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
