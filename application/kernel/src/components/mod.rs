mod analytics;
mod kernel;
mod utils;

use anyhow::Result;

use crate::components::analytics::Analytics;
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
    Analytics {
        pause: pause.clone(),
    }
    .run(&config)?;
    Kernel { pause }.run(&config)?;
    Ok(())
}
