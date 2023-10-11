use std::fmt::{self, write};

use colored::Colorize;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TypeKind {
    Invalid = 0,
    Standard = 1,
    Fire = 2,
    Water = 3,
    Nature = 4,
    Electric = 5,
    Air = 6,
    Ground = 7,
    Metal = 8,
    Light = 9,
    Dark = 10,
    Dragon = 11
}

pub const TYPE_COUNT: u32 = 11;

impl From<u32> for TypeKind {
    fn from(value: u32) -> Self {
        return match value {
            0 => TypeKind::Invalid,
            1 => TypeKind::Standard,
            2 => TypeKind::Fire,
            3 => TypeKind::Water,
            4 => TypeKind::Nature,
            5 => TypeKind::Electric,
            6 => TypeKind::Air,
            7 => TypeKind::Ground,
            8 => TypeKind::Metal,
            9 => TypeKind::Light,
            10 => TypeKind::Dark,
            11 => TypeKind::Dragon,
            _ => panic!("Invalid type id: {}", value),
        };
    }
}

impl fmt::Debug for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeKind::Invalid => panic!("Cannot fmt invalid type"),
            TypeKind::Standard => write!(f, "{}", "Standard".truecolor(200, 200, 200)),
            TypeKind::Fire => write!(f, "{}", "Fire".truecolor(209, 72, 13)),
            TypeKind::Water => write!(f, "{}", "Water".truecolor(6, 106, 189)),
            TypeKind::Nature => write!(f, "{}", "Nature".truecolor(94, 201, 22)),
            TypeKind::Electric => write!(f, "{}", "Electric".truecolor(227, 221, 102)),
            TypeKind::Air => write!(f, "{}", "Air".truecolor(191, 242, 227)),
            TypeKind::Ground => write!(f, "{}", "Ground".truecolor(156, 115, 11)),
            TypeKind::Metal => write!(f, "{}", "Metal".truecolor(191, 184, 185)),
            TypeKind::Light => write!(f, "{}", "Light".truecolor(233, 247, 203)),
            TypeKind::Dark => write!(f, "{}", "Dark".truecolor(40, 3, 61)),
            TypeKind::Dragon => write!(f, "{}", "Dragon".truecolor(92, 76, 199)),
        } 
    }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}
