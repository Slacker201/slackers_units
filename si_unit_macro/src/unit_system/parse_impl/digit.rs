use std::fmt::Display;

use crate::unit_system::Digit;

impl Digit {
    pub fn from_char(c: char) -> Option<Self> {
        let ret = match c {
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            _ => return None,
        };
        Some(ret)
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Digit::Zero => '0',
                Digit::One => '1',
                Digit::Two => '2',
                Digit::Three => '3',
                Digit::Four => '4',
                Digit::Five => '5',
                Digit::Six => '6',
                Digit::Seven => '7',
                Digit::Eight => '8',
                Digit::Nine => '9',
            }
        )
    }
}
