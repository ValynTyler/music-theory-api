use std::ops;

use crate::Semitones;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Octave(pub i8);

impl Octave {
    pub fn semitones(&self) -> Semitones {
        self.0 * 12
    }
}

impl ops::Add for Octave {
    type Output = Octave;

    fn add(self, rhs: Self) -> Self::Output {
        Octave(self.0 + rhs.0)
    }
}

impl ops::Add<i8> for Octave {
    type Output = Octave;

    fn add(self, rhs: i8) -> Self::Output {
        Octave(self.0 + rhs)
    }
}

impl ops::Sub<i8> for Octave {
    type Output = Octave;

    fn sub(self, rhs: i8) -> Self::Output {
        Octave(self.0 - rhs)
    }
}
