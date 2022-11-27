use raestro::prelude::Channels;

#[cfg_attr(not(release), derive(Debug))]
#[derive(PartialEq, Eq)]


pub enum Grip {
    HAMMER,
    CUP,
    FLAT
}

impl Default for Grip {
    fn default() -> Self {
        Self::FLAT
    }
}

#[cfg(feature = "pseudo_analytics")]
impl From<f64> for Grip {
    fn from(data: f64) -> Self {
        const MODULO_BASE: u64 = 3;
        let data = data.floor() as u64;
        let data = data % MODULO_BASE;
        match data {
            0 => Self::HAMMER,
            1 => Self::CUP,
            _ => Self::FLAT,
        }
    }
}
