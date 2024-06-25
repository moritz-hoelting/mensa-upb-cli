use std::fmt::Display;

use clap::ValueEnum;
use const_format::concatcp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mensa {
    Forum,
    Academica,
    Picknick,
    BonaVista,
    GrillCafe,
    ZM2,
    Basilica,
    Atrium,
}

const POST_URL_BASE: &str = "https://www.studierendenwerk-pb.de/gastronomie/speiseplaene/";

impl Mensa {
    pub fn get_url(&self) -> &str {
        match self {
            Self::Forum => concatcp!(POST_URL_BASE, "forum/"),
            Self::Academica => concatcp!(POST_URL_BASE, "mensa-academica/"),
            Self::Picknick => concatcp!(POST_URL_BASE, "picknick/"),
            Self::BonaVista => concatcp!(POST_URL_BASE, "bona-vista/"),
            Self::GrillCafe => concatcp!(POST_URL_BASE, "grillcafe/"),
            Self::ZM2 => concatcp!(POST_URL_BASE, "mensa-zm2/"),
            Self::Basilica => concatcp!(POST_URL_BASE, "mensa-basilica-hamm/"),
            Self::Atrium => concatcp!(POST_URL_BASE, "mensa-atrium-lippstadt/"),
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Self::Forum => 'F',
            Self::Academica => 'A',
            Self::Picknick => 'P',
            Self::BonaVista => 'B',
            Self::GrillCafe => 'G',
            Self::ZM2 => 'Z',
            Self::Basilica => 'H',
            Self::Atrium => 'L',
        }
    }
}

impl Display for Mensa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Forum => "Forum",
            Self::Academica => "Academica",
            Self::Picknick => "Picknick",
            Self::BonaVista => "Bona Vista",
            Self::GrillCafe => "Grill | Café",
            Self::ZM2 => "ZM2",
            Self::Basilica => "Basilica",
            Self::Atrium => "Atrium",
        };
        f.write_str(s)
    }
}
