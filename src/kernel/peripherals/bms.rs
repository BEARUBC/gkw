use std::io;

use crate::kernel::kernel;
use crate::kernel::peripherals::Percentage;
use crate::kernel::Kernel;

/// Bms (Battery-Management-System) state.
///
/// Holds state for information pertinent to on-system battery.
#[allow(unused)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Bms {
    percentage: Percentage,
}

impl Kernel {
    /// Gets the battery percentage of the on-board battery.
    ///
    /// This function does *NOT* read battery-percentage from the BMS; it only returns the cached
    /// percentage from the latest read.
    #[allow(unused)]
    pub fn get_percentage() -> io::Result<Percentage> {
        kernel().map(|krn| krn.bms.percentage)
    }

    /// Reads battery percentage from BMS peripheral and updates kernel state.
    ///
    /// Performs a read from the actual BMS and caches result inside of the kernel.
    ///
    /// Furthermore, depending on the percentage read, this function will put the kernel into its
    /// appropriate state (i.e., if battery was charged during previous read and kernel was in the
    /// `Active` state, but a current read returns a low-battery percentage, the kernel will be
    /// transitioned to the `Safety` state).
    #[allow(unused)]
    pub fn read_percentage() -> io::Result<Percentage> {
        todo!()
    }
}
