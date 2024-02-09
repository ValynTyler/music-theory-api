use core::fmt;

use crate::Semitones;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PitchClass {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl PitchClass {
    /// Returns the `PitchClass` `idx`-1 positions above `self`
    /// (e.g: `G.next() -> A`)
    pub fn next(&self, idx: u8) -> PitchClass {
        let mut pitch_class: PitchClass = *self;
        for _ in 0..idx {
            pitch_class = match pitch_class {
                Self::C => Self::D,
                Self::D => Self::E,
                Self::E => Self::F,
                Self::F => Self::G,
                Self::G => Self::A,
                Self::A => Self::B,
                Self::B => Self::C,
            }
        }
        pitch_class
    }

    /// Returns the `PitchClass` `idx` positions below `self`
    /// (e.g: `G.next() -> F`)
    pub fn prev(&self, idx: u8) -> PitchClass {
        let mut pitch_class: PitchClass = *self;
        for _ in 0..idx {
            pitch_class = match pitch_class {
                Self::C => Self::B,
                Self::D => Self::C,
                Self::E => Self::D,
                Self::F => Self::E,
                Self::G => Self::F,
                Self::A => Self::G,
                Self::B => Self::A,
            }
        }
        pitch_class
    }

    /// Returns the value of the given `PitchClass`
    /// in `Semitones`. (The distance away from C)
    pub fn semitones(&self) -> Semitones {
        match self {
            PitchClass::C => 0,
            PitchClass::D => 2,
            PitchClass::E => 4,
            PitchClass::F => 5,
            PitchClass::G => 7,
            PitchClass::A => 9,
            PitchClass::B => 11,
        }
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PitchClass::C => "C",
                PitchClass::D => "D",
                PitchClass::E => "E",
                PitchClass::F => "F",
                PitchClass::G => "G",
                PitchClass::A => "A",
                PitchClass::B => "B",
            }
        )
    }
}
