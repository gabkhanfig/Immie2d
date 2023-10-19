use std::fmt::{self};

use colored::Colorize;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ElementKind {
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

pub const ELEMENT_COUNT: u32 = 11;

impl From<u32> for ElementKind {
    fn from(value: u32) -> Self {
        return match value {
            0 => ElementKind::Invalid,
            1 => ElementKind::Standard,
            2 => ElementKind::Fire,
            3 => ElementKind::Water,
            4 => ElementKind::Nature,
            5 => ElementKind::Electric,
            6 => ElementKind::Air,
            7 => ElementKind::Ground,
            8 => ElementKind::Metal,
            9 => ElementKind::Light,
            10 => ElementKind::Dark,
            11 => ElementKind::Dragon,
            _ => panic!("Invalid type id: {}", value),
        };
    }
}

impl fmt::Debug for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ElementKind::Invalid => panic!("Cannot fmt invalid type"),
            ElementKind::Standard => write!(f, "{}", "Standard".truecolor(200, 200, 200)),
            ElementKind::Fire => write!(f, "{}", "Fire".truecolor(209, 72, 13)),
            ElementKind::Water => write!(f, "{}", "Water".truecolor(6, 106, 189)),
            ElementKind::Nature => write!(f, "{}", "Nature".truecolor(94, 201, 22)),
            ElementKind::Electric => write!(f, "{}", "Electric".truecolor(227, 221, 102)),
            ElementKind::Air => write!(f, "{}", "Air".truecolor(191, 242, 227)),
            ElementKind::Ground => write!(f, "{}", "Ground".truecolor(156, 115, 11)),
            ElementKind::Metal => write!(f, "{}", "Metal".truecolor(191, 184, 185)),
            ElementKind::Light => write!(f, "{}", "Light".truecolor(233, 247, 203)),
            ElementKind::Dark => write!(f, "{}", "Dark".truecolor(40, 3, 61)),
            ElementKind::Dragon => write!(f, "{}", "Dragon".truecolor(92, 76, 199)),
        } 
    }
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}
