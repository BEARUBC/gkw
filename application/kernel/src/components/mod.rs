mod bms;
mod emg;
mod kernel;
mod utils;

use anyhow::Result;
#[cfg(feature = "simulation")]
use crossbeam::channel::bounded;

#[cfg(feature = "simulation")]
use crate::components::bms::Bms;
#[cfg(feature = "simulation")]
use crate::components::emg::Emg;
#[cfg(feature = "simulation")]
use crate::components::kernel::Kernel;
use crate::config::Config;
#[cfg(feature = "simulation")]
use crate::wait::Wait;

#[cfg(feature = "simulation")]
const TCP_BUFFER_CAPACITY: usize = 256;

trait Component {
    fn run(self, _: &Config) -> Result<()>;
}

#[cfg(feature = "simulation")]
pub(super) fn run(config: Config) -> Result<()> {
    let (tx, rx) = bounded(config.components.kernel.msg_queue_length);
    let pause = Wait::default();
    let emg = Emg {
        tx: tx.clone(),
        pause: pause.clone(),
    };
    let bms = Bms { tx };
    let kernel = Kernel {
        emg_data: 0.0,
        pause,
        rx,
    };
    emg.run(&config)?;
    bms.run(&config)?;
    kernel.run(&config)?;
    Ok(())
}

#[cfg(not(feature = "simulation"))]
pub(super) fn run(_: Config) -> Result<()> {
    todo!()
}
