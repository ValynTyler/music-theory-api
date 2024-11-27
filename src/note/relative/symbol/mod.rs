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
