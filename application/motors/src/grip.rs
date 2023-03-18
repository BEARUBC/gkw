#[derive(PartialEq, Eq, Copy, Clone)]
#[cfg_attr(not(release), derive(Debug))]
pub enum Grip {
    Hammer{x: f64},
    Cup{x: f64},
    Flat{x: f64},
}

impl From<Grip> for [u16; 3usize] {
    fn from(grip: Grip) -> [u16; 3usize] {
        match grip {
            Grip::Hammer{x} => [4000*x, 4000*x, 4000*x],
            Grip::Cup{x} => [5000*x, 5000*x, 5000*x],
            Grip::Flat{x} => [6000*x, 6000*x, 6000*x],
        }
    }
}

impl Default for Grip {
    fn default() -> Self {
        Self::Flat
    }
}

impl From<f16> for Grip {
    fn from(data: f16) -> Self {
        const MODULO_BASE: u16 = 3;
        let grip = (data.floor() as u64) % MODULO_BASE;
        let scale = 2*(data - grip);
        match grip {
            0 => Self::Hammer{x: scale},
            1 => Self::Cup{x: scale},
            _ => Self::Flat{x: scale},
        }
    }
}