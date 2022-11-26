mod bms;
mod kernel;
mod utils;

use anyhow::Result;

use crate::components::bms::Bms;
use crate::components::kernel::Kernel;
use crate::config::Config;
use crate::wait::Wait;

#[cfg(feature = "tcp_edge")]
const TCP_BUFFER_CAPACITY: usize = 256;

trait Component {
    fn run(self, _: &Config) -> Result<()>;
}

pub(super) fn run(config: Config) -> Result<()> {
    let pause = Wait::default();
    Kernel {
        pause: pause.clone(),
    }
    .run(&config)?;
    Bms { pause }.run(&config)?;
    Ok(())
}
