#[derive(PartialEq, Eq, Copy, Clone)]
#[cfg_attr(not(release), derive(Debug))]
pub enum Grip {
    Hammer,
    Cup,
    Flat,
}

impl From<Grip> for [u16; 3usize] {
    fn from(grip: Grip) -> [u16; 3usize] {
        match grip {
            Grip::Hammer => [4000, 4000, 4000],
            Grip::Cup => [5000, 5000, 5000],
            Grip::Flat => [6000, 6000, 6000],
        }
    }
}

impl Default for Grip {
    fn default() -> Self {
        Self::Flat
    }
}

#[cfg(feature = "pseudo_analytics")]
impl From<u16> for Grip {
    fn from(data: u16) -> Self {
        const MODULO_BASE: u16 = 3;
        let data = data.floor() as u64;
        let data = data % MODULO_BASE;
        match data {
            0 => Self::Hammer,
            1 => Self::Cup,
            _ => Self::Flat,
        }
    }
}