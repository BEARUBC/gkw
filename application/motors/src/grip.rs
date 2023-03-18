#[derive(PartialEq, Copy, Clone)]
pub enum Grip {
    Hammer{x: f64},
    Cup{x: f64},
    Flat{x: f64},
}

impl From<Grip> for [u16; 3usize] {
    fn from(grip: Grip) -> [u16; 3usize] {
        match grip {
            Grip::Hammer{x} => [((4000 as f64)*x).floor() as u16, ((4000 as f64)*x).floor() as u16, ((4000 as f64)*x).floor() as u16],
            Grip::Cup{x} => [((5000 as f64)*x).floor() as u16, ((5000 as f64)*x).floor() as u16, ((5000 as f64)*x).floor() as u16],
            Grip::Flat{x} => [((6000 as f64)*x).floor() as u16, ((6000 as f64)*x).floor() as u16, ((6000 as f64)*x).floor() as u16],
        }
    }
}

impl Default for Grip {
    fn default() -> Self {
        Self::Flat{x: 1.0 as f64}
    }
}

impl From<f64> for Grip {
    fn from(data: f64) -> Self {
        const MODULO_BASE: u16 = 3;
        let grip = (data.floor() as u16) % MODULO_BASE;
        let scale = 2.0*(data - grip as f64);
        match grip {
            0 => Self::Hammer{x: scale},
            1 => Self::Cup{x: scale},
            _ => Self::Flat{x: scale},
        }
    }
}