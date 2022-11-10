mod bms;
mod emg;
mod kernel;
mod utils;

use anyhow::Result;
use crossbeam::channel::bounded;

use crate::components::bms::Bms;
use crate::components::emg::Emg;
use crate::components::kernel::Kernel;
use crate::config::Config;
use crate::wait::Wait;

#[cfg(feature = "tcp_data")]
const TCP_BUFFER_CAPACITY: usize = 256;

trait Component {
    fn run(self, _: &Config) -> Result<()>;
}

pub(super) fn run(config: Config) -> Result<()> {
    let (tx, rx) = bounded(kernel::MESSAGE_CAPACITY);
    let pause = Wait::default();
    Emg::new(tx.clone(), pause.clone()).run(&config)?;
    Bms::new(tx).run(&config)?;
    Kernel::new(pause, rx)?.run(&config)?;
    Ok(())
}
