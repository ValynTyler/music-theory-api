use crate::step::{IStepNonZero, UStepNonZero};

pub enum NoteSymbol {
    Sharp(UStepNonZero),
    Flat(UStepNonZero),
    Natural,
}
