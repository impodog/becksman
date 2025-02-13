use crate::prelude::*;
use becks_crew::*;

impl Repr for Gender {
    fn repr(&self) -> &'static str {
        match self {
            Gender::Male => assets::TEXT.get("gender_male"),
            Gender::Female => assets::TEXT.get("gender_female"),
        }
    }

    fn all() -> &'static [Self] {
        &[Gender::Male, Gender::Female]
    }
}

impl Repr for Social {
    fn repr(&self) -> &'static str {
        match self {
            Social::Teacher => assets::TEXT.get("social_teacher"),
            Social::Student => assets::TEXT.get("social_student"),
        }
    }

    fn all() -> &'static [Self] {
        &[Social::Student, Social::Teacher]
    }
}

impl Repr for Clothes {
    fn repr(&self) -> &'static str {
        match self {
            Clothes::S => "S",
            Clothes::M => "M",
            Clothes::L => "L",
            Clothes::XL => "XL",
            Clothes::XXL => "XXL",
            Clothes::XXXL => "XXXL",
        }
    }

    fn all() -> &'static [Self] {
        &[
            Clothes::S,
            Clothes::M,
            Clothes::L,
            Clothes::XL,
            Clothes::XXL,
            Clothes::XXXL,
        ]
    }
}

impl Repr for Hand {
    fn repr(&self) -> &'static str {
        match self {
            Hand::Right => assets::TEXT.get("hand_right"),
            Hand::Left => assets::TEXT.get("hand_left"),
        }
    }

    fn all() -> &'static [Self] {
        &[Hand::Right, Hand::Left]
    }
}

impl Repr for Hold {
    fn repr(&self) -> &'static str {
        match self {
            Hold::Horiz => assets::TEXT.get("hold_horiz"),
            Hold::Verti => assets::TEXT.get("hold_verti"),
        }
    }

    fn all() -> &'static [Self] {
        &[Hold::Horiz, Hold::Verti]
    }
}

pub enum Brand {
    DHS,
    YH,
    Butterfly,
    Stiga,
    XIOM,
    YOOLA,
}

impl Repr for Brand {
    fn repr(&self) -> &'static str {
        match self {
            Brand::DHS => assets::TEXT.get("paddlebrand_dhs"),
            Brand::YH => assets::TEXT.get("paddlebrand_yh"),
            Brand::Butterfly => assets::TEXT.get("paddlebrand_butterfly"),
            Brand::Stiga => assets::TEXT.get("paddlebrand_stiga"),
            Brand::XIOM => assets::TEXT.get("paddlebrand_xoim"),
            Brand::YOOLA => assets::TEXT.get("paddlebrand_yoola"),
        }
    }

    fn all() -> &'static [Self] {
        &[
            Self::DHS,
            Self::YH,
            Self::Butterfly,
            Self::Stiga,
            Self::XIOM,
            Self::YOOLA,
        ]
    }
}

impl ClientRepr for Brand {
    fn from_server(s: &str) -> Self {
        match s {
            "dhs" => Brand::DHS,
            "yh" => Brand::YH,
            "but" => Brand::Butterfly,
            "sti" => Brand::Stiga,
            "xio" => Brand::XIOM,
            "yoo" => Brand::YOOLA,
            _ => Brand::DHS,
        }
    }

    fn to_server(&self) -> String {
        let s = match self {
            Self::DHS => "dhs",
            Self::YH => "yh",
            Self::Butterfly => "but",
            Self::Stiga => "sti",
            Self::XIOM => "xio",
            Self::YOOLA => "yoo",
        };
        s.to_owned()
    }
}
