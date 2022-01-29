pub mod peripherals;

use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use lazy_static::lazy_static;

use crate::error::ToIoError;
use crate::kernel::peripherals::bms::Bms;
use crate::kernel::peripherals::fingers::Fingers;
use crate::kernel::peripherals::wrist::Wrist;

lazy_static! {
    /// A static kernel reference which can be (thread-safely) referenced and mutated.
    /// This represents a singleton instance in the entire system.
    ///
    /// All `Kernel` functions (i.e., all functions residing in `impl Kernel { ... }`) will internally refer to and/or mutate this object.
    static ref KERNEL: Arc<Mutex<Kernel>> = Arc::new(Mutex::new(Kernel::default()));
}

/// A transition matrix for the internal state-machine.
///
/// Each row represents each corresponding state.
/// Each element in each row represents whether or not that other state can transition to it.
///
/// For example:
/// ``` ignore
/// let state_a: State = ... as usize;
/// let state_b: State = ... as usize;
///
/// // can transition from `state_b -> state_a`;
/// let can_transition: bool = TRANSITION[state_a][state_b];
/// ```
///
/// Note that you should always be able to transition from `current_state -> current_state`.
const TRANSITION: [[bool; NUMBER_OF_STATES]; NUMBER_OF_STATES] = [
    [true, false, false, false],
    [false, true, false, false],
    [false, false, true, false],
    [false, false, false, true],
];

/// The number of states present in the internal state-machine.
///
/// Todo!:
/// Have this number automatically reflect the number of variants in State.
const NUMBER_OF_STATES: usize = 4usize;

/// Representation of all states inhabitable by the GKW software.
///
/// Note that each state variant is (internally) implemented as a number, similar to C-enums.
/// The numbering of each variant has no significance; it is used solely for indexing purposes.
/// (I.e., it is not the case that just because a state has a higher number, it is "more important"
/// or anything).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    /// Safety:
    /// Used for when the arm is in a potentially harmful/dangerous state (i.e., low-battery).
    ///
    /// Limited API execution; limited capabilities in general.
    #[allow(unused)]
    Safety = 0,

    /// Active:
    /// Used for when the arm is fully operational/functional.
    ///
    /// Full access to all APIs exposed by the kernel; full capabilities in general.
    #[allow(unused)]
    Active = 1,

    /// Failure:
    /// Used for when the arm -or a component/peripheral of the arm- has failed.
    ///
    /// Extremely limited API execution; possibility of a system-restart should be considered at
    /// this point.
    #[allow(unused)]
    Failure = 2,

    /// Init:
    /// Used for when the arm is initially booting up, and is performing booting up
    /// procedures/scripts.
    ///
    /// APIs are not exposed at this point since booting up is still occurring.
    Init = 3,
}

/// When the GKW software boots up, the starting, default kernel state should ALWAYS be
/// `State::Init`.
impl Default for State {
    fn default() -> Self {
        Self::Init
    }
}

/// StateMachine for internal usage.
/// Represents the current state of the GKW software.
///
/// All APIs exposed by the kernel should be first routed through this class.
#[derive(Default, Debug, Clone, Copy)]
pub struct Kernel {
    /// Internal state of the kernel.
    state: State,

    /// Fingers state-management.
    #[allow(unused)]
    fingers: Fingers,

    /// Wrist state-management.
    #[allow(unused)]
    wrist: Wrist,

    /// Bms state-management.
    bms: Bms,
}

/// Private implementations for internal usage by the `Kernel` class only.
impl Kernel {
    /// Transitions kernel from its current internal state to another.
    /// Returns `true` iff the next-state is transitionable from the current-state.
    /// Returns `false` otherwise.
    ///
    /// The transition function should only be callable from `Kernel` functions.
    /// Third-parties should *not* be able to request state-transitions (i.e., state should be
    /// something that is strictly abstracted away from external users).
    #[allow(unused)]
    fn try_transition(next_state: State) -> io::Result<bool> {
        let mut krn = kernel()?;

        let curr = krn.state as usize;
        let next = next_state as usize;

        if curr == next {
            return Ok(true);
        }

        let can_transition = TRANSITION[next][curr];

        if can_transition {
            krn.state = next_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Gets current state of Kernel.
    ///
    /// Third-parties should *not* be able to request state-transitions (i.e., state should be
    /// something that is strictly abstracted away from external users).
    #[allow(unused)]
    fn state() -> io::Result<State> {
        kernel().map(|kernel| kernel.state)
    }
}

/// Convenience function to get a clone of KERNEL.
fn kernel() -> io::Result<MutexGuard<'static, Kernel>> {
    KERNEL.lock().map_err(|err| {
        err.to_io_error("Something went wrong while trying to retrieve the kernel state.")
    })
}
