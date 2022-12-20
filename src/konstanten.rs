use clap::ValueEnum;

const A1_LANG: &str = "A.1 - XXXXXXX";
const B2_LANG: &str = "B.2 - XXXXXXX";

pub const A1: &str = "A.1";
pub const B2: &str = "B.2";

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum UmwertungsTabelle {
    Iso18265A1,
    Iso18265B2,
}

impl UmwertungsTabelle {
    pub fn enum_to_bezeichner(tabelle: UmwertungsTabelle) -> &'static str {
        match tabelle {
            UmwertungsTabelle::Iso18265A1 => A1_LANG,
            UmwertungsTabelle::Iso18265B2 => B2_LANG,
        }
    }

    pub fn enum_to_kurzbezeichner(tabelle: UmwertungsTabelle) -> &'static str {
        match tabelle {
            UmwertungsTabelle::Iso18265A1 => A1,
            UmwertungsTabelle::Iso18265B2 => B2,
        }
    }

    pub fn bezeichner_to_enum(enumbezeichner: &str) -> Option<UmwertungsTabelle> {
        match enumbezeichner {
            A1_LANG => Some(UmwertungsTabelle::Iso18265A1),
            B2_LANG => Some(UmwertungsTabelle::Iso18265B2),

            _ => None,
        }
    }

    pub fn kurzbezeichner_to_enum(enumbezeichner: &str) -> Option<UmwertungsTabelle> {
        match enumbezeichner {
            A1 => Some(UmwertungsTabelle::Iso18265A1),
            B2 => Some(UmwertungsTabelle::Iso18265B2),

            _ => None,
        }
    }

    pub fn bezeichner() -> Vec<&'static str> {
        vec![
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265A1),
            UmwertungsTabelle::enum_to_bezeichner(UmwertungsTabelle::Iso18265B2),
        ]
    }
}
