use std::fmt::Display;

use crate::step::{IStep, UStep, UStepNonZero};

pub enum NoteSymbol {
    Sharp(UStepNonZero),
    Flat(UStepNonZero),
    Natural,
}

impl From<NoteSymbol> for IStep {
    fn from(value: NoteSymbol) -> Self {
        match value {
            NoteSymbol::Sharp(ustep_non_zero) => IStep::from(UStep::from(ustep_non_zero)),
            NoteSymbol::Flat(ustep_non_zero) => IStep::from(UStep::from(ustep_non_zero)),
            NoteSymbol::Natural => IStep::from(0),
        }
    }
}

impl Display for NoteSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteSymbol::Sharp(ustep_non_zero) => {
                for _ in 0..u8::from(*ustep_non_zero) / 2 {
                    write!(f, "𝄪")?
                }
                for _ in 0..u8::from(*ustep_non_zero) % 2 {
                    write!(f, "♯")?
                }
            },
            NoteSymbol::Flat(ustep_non_zero) => {
                for _ in 0..u8::from(*ustep_non_zero) / 2 {
                    write!(f, "𝄫")?
                }
                for _ in 0..u8::from(*ustep_non_zero) % 2 {
                    write!(f, "♭")?
                }
            },
            NoteSymbol::Natural => write!(f, "")?, // (♮)
        }
        Ok(())
    }
}
