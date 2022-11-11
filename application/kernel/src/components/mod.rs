mod analytics;
mod bms;
mod emg;
mod kernel;
mod utils;

use anyhow::Result;
use crossbeam::channel::bounded;

use crate::components::analytics::Analytics;
use crate::components::bms::Bms;
use crate::components::emg::Emg;
use crate::components::kernel::Kernel;
use crate::config::Config;
use crate::wait::Wait;

#[cfg(feature = "tcp_edge")]
const TCP_BUFFER_CAPACITY: usize = 256;

trait Component {
    fn run(self, _: &Config) -> Result<()>;
}

pub(super) fn run(config: Config) -> Result<()> {
    let (tx, rx) = bounded(kernel::MESSAGE_CAPACITY);
    let (analytics_tx, analytics_rx) = bounded(analytics::MESSAGE_CAPACITY);
    let pause = Wait::default();
    Emg {
        tx: tx.clone(),
        pause: pause.clone(),
    }
    .run(&config)?;
    Bms { tx: tx.clone() }.run(&config)?;
    Analytics {
        tx,
        rx: analytics_rx,
    }
    .run(&config)?;
    Kernel {
        pause,
        analytics_tx,
        rx,
    }
    .run(&config)?;
    Ok(())
}
