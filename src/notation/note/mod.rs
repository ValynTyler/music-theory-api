mod solfege;
mod letter;

pub use solfege::*;
pub use letter::*;

impl From::<Solfege> for Letter {
    fn from(value: Solfege) -> Self {
        match value {
            Solfege::Do  => Letter::C,
            Solfege::Re  => Letter::D,
            Solfege::Mi  => Letter::E,
            Solfege::Fa  => Letter::F,
            Solfege::Sol => Letter::G,
            Solfege::La  => Letter::A,
            Solfege::Si  => Letter::B,
        }
    }
}

impl From<Letter> for Solfege {
    fn from(value: Letter) -> Self {
        match value {
            Letter::A => Solfege::La,
            Letter::B => Solfege::Si,
            Letter::C => Solfege::Do,
            Letter::D => Solfege::Re,
            Letter::E => Solfege::Mi,
            Letter::F => Solfege::Fa,
            Letter::G => Solfege::Sol,
        }
    }
}
