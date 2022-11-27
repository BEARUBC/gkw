use raestro::prelude::Channels;

#[cfg_attr(not(release), derive(Debug))]
#[derive(PartialEq, Eq)]


pub enum Grip {
    HAMMER,
    CUP,
    FLAT,
}

impl From<Grip> for [u16; 3usize] {
    fn from(g: Grip) -> [u16; 3usize] {
        match g {
            Grip::HAMMER => return [300, 150, 100],
            Grip::CUP => return [150, 300, 100],
            Grip::FLAT => return [50, 100, 300],
        }
    }
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
