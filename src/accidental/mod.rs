use core::fmt;
use std::ops;

use crate::Semitones;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Accidental(pub Semitones);

impl Accidental {
    pub const NATURAL: Accidental = Accidental(0);
    pub const SHARP: Accidental = Accidental(1);
    pub const FLAT: Accidental = Accidental(-1);

    pub const BECAR: Accidental = Accidental(0);
    pub const DIEZ: Accidental = Accidental(1);
    pub const BEMOL: Accidental = Accidental(-1);
}

impl Accidental {
    pub fn semitones(&self) -> Semitones {
        self.0
    }

    pub fn is_sharp(&self) -> bool {
        self.0 > 0
    }

    pub fn is_flat(&self) -> bool {
        self.0 < 0
    }
}

impl ops::Add for Accidental {
    type Output = Accidental;

    fn add(self, rhs: Self) -> Self::Output {
        Accidental(self.0 + rhs.0)
    }
}

impl ops::Sub for Accidental {
    type Output = Accidental;

    fn sub(self, rhs: Self) -> Self::Output {
        Accidental(self.0 - rhs.0)
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let mut s: String = String::new();
        
        for _ in 0..self.0.abs() {
            if self.0 > 0 {
                s += "#";
            } else {
                s += "b";
            }
        }

        write!(f, "{}", s)
    }
}
