pub mod api;

use std::io;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref KERNEL: Kernel = Kernel::default();
}

/// A transition matrix for the internal state-machine.
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
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
    /// APIs are not exposed at this point, since booting up is still occurring.
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
/// All APIs should be first routed through this class
#[derive(Default)]
pub struct Kernel {
    current_state: State,
}

impl Kernel {
    /// Transitions a kernel from its current internal state to another.
    /// Throws an error if the state provided is cannot be transitioned to from the current state.
    #[allow(unused)]
    pub fn transition(&mut self, #[allow(unused)] next_state: State) -> io::Result<()> {
        let curr = self.current_state as usize;
        let next = next_state as usize;

        let can_transition = TRANSITION[next][curr];

        if can_transition {
            self.current_state = next_state;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid transition",
            ))
        }
    }
}
