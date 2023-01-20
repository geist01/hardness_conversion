use clap::ValueEnum;

const A1_LANG: &str = "A.1 - Unalloyed and low-alloyed steels and cast steel";
const B2_LANG: &str = "B.2 - Heat-treatable steels in tempered state";
const B3_LANG: &str = "B.3 - Hardened steels in untreated, annealed or normalized state";
const B4_LANG: &str = "B.4 - Heat-treatable steels in hardened state";
const C2_LANG: &str = "C.2 - Cold work steels";
const D2_LANG: &str = "D.2 - High-speed steels";
const F2_LANG: &str = "F.2 - Cartridge brass";

pub const A1: &str = "A.1";
pub const B2: &str = "B.2";
pub const B3: &str = "B.3";
pub const B4: &str = "B.4";
pub const C2: &str = "C.2";
pub const D2: &str = "D.2";
pub const F2: &str = "F.2";

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum UmwertungsTabelle {
    Iso18265A1,
    Iso18265B2,
    Iso18265B3,
    Iso18265B4,
    Iso18265C2,
    Iso18265D2,
    Iso18265F2,
}

impl UmwertungsTabelle {
    pub fn enum_to_bezeichner(tabelle: UmwertungsTabelle) -> &'static str {
        match tabelle {
            UmwertungsTabelle::Iso18265A1 => A1_LANG,
            UmwertungsTabelle::Iso18265B2 => B2_LANG,
            UmwertungsTabelle::Iso18265B3 => B3_LANG,
            UmwertungsTabelle::Iso18265B4 => B4_LANG,
            UmwertungsTabelle::Iso18265C2 => C2_LANG,
            UmwertungsTabelle::Iso18265D2 => D2_LANG,
            UmwertungsTabelle::Iso18265F2 => F2_LANG,
        }
    }

    pub fn enum_to_kurzbezeichner(tabelle: UmwertungsTabelle) -> &'static str {
        match tabelle {
            UmwertungsTabelle::Iso18265A1 => A1,
            UmwertungsTabelle::Iso18265B2 => B2,
            UmwertungsTabelle::Iso18265B3 => B3,
            UmwertungsTabelle::Iso18265B4 => B4,
            UmwertungsTabelle::Iso18265C2 => C2,
            UmwertungsTabelle::Iso18265D2 => D2,
            UmwertungsTabelle::Iso18265F2 => F2,
        }
    }

    pub fn bezeichner_to_enum(enumbezeichner: &str) -> Option<UmwertungsTabelle> {
        match enumbezeichner {
            A1_LANG => Some(UmwertungsTabelle::Iso18265A1),
            B2_LANG => Some(UmwertungsTabelle::Iso18265B2),
            B3_LANG => Some(UmwertungsTabelle::Iso18265B3),
            B4_LANG => Some(UmwertungsTabelle::Iso18265B4),
            C2_LANG => Some(UmwertungsTabelle::Iso18265C2),
            D2_LANG => Some(UmwertungsTabelle::Iso18265D2),
            F2_LANG => Some(UmwertungsTabelle::Iso18265F2),
            _ => None,
        }
    }

    pub fn kurzbezeichner_to_enum(enumbezeichner: &str) -> Option<UmwertungsTabelle> {
        match enumbezeichner {
            A1 => Some(UmwertungsTabelle::Iso18265A1),
            B2 => Some(UmwertungsTabelle::Iso18265B2),
            B3 => Some(UmwertungsTabelle::Iso18265B3),
            B4 => Some(UmwertungsTabelle::Iso18265B4),
            C2 => Some(UmwertungsTabelle::Iso18265C2),
            D2 => Some(UmwertungsTabelle::Iso18265D2),
            F2 => Some(UmwertungsTabelle::Iso18265F2),
            _ => None,
        }
    }

    pub fn bezeichner() -> Vec<&'static str> {
        vec![
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265A1),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265B2),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265B3),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265B4),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265C2),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265D2),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265F2),

        ]
    }
}
