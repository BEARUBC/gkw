#[derive(PartialEq, Eq)]
#[cfg_attr(not(release), derive(Debug))]
pub enum Grip {
    Hammer,
    Cup,
    Flat,
}

impl From<Grip> for [u16; 3usize] {
    fn from(grip: Grip) -> [u16; 3usize] {
        match grip {
            Grip::Hammer => [300, 150, 100],
            Grip::Cup => [150, 300, 100],
            Grip::Flat => [50, 100, 300],
        }
    }
}

impl Default for Grip {
    fn default() -> Self {
        Self::Flat
    }
}

#[cfg(feature = "pseudo_analytics")]
impl From<f64> for Grip {
    fn from(data: f64) -> Self {
        const MODULO_BASE: u64 = 3;
        let data = data.floor() as u64;
        let data = data % MODULO_BASE;
        match data {
            0 => Self::Hammer,
            1 => Self::Cup,
            _ => Self::Flat,
        }
    }
}
