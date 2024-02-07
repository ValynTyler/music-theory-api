use crate::Semitones;

#[derive(Copy, Clone, Debug)]
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
    /// Returns the `PitchClass` succeeding `self`
    /// (e.g: `G.next() -> A`)
    pub fn next(&self) -> PitchClass {
        match self {
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::A,
            Self::A => Self::B,
            Self::B => Self::C,
        }
    }

    /// Returns the `PitchClass` proceeding `self`
    /// (e.g: `G.next() -> F`)
    pub fn prev(&self) -> PitchClass {
        match self {
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
            Self::A => Self::G,
            Self::B => Self::A,
        }
    }

    /// Returns the `PitchClass` that is `dist` positions above
    /// (or below) it - depending on the sign
    pub fn cycle(&self, mut dist: i8) -> PitchClass {
        dist %= 7;

        if dist < 0 {
            dist = 7 + dist;
        }

        let mut current = *self;
        for _ in 0..dist {
            current = current.next();
        }
        
        current
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