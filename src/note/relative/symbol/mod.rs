use std::fmt::Display;

use crate::step::{IStep, UStep, UStepNonZero};

#[derive(Clone, Copy)]
pub enum NoteSymbol {
    Sharp(UStepNonZero),
    Flat(UStepNonZero),
    Natural,
}

#[macro_export]
macro_rules! sharp {
    ($value:literal) => {{
        use $crate::note::relative::symbol::NoteSymbol;
        use $crate::step::UStepNonZero;
        NoteSymbol::Sharp(UStepNonZero::try_from(u8::try_from($value).unwrap()).unwrap())
    }};

    () => { sharp!(1) };
}

#[macro_export]
macro_rules! flat {
    ($value:literal) => {{
        use $crate::note::relative::symbol::NoteSymbol;
        use $crate::step::UStepNonZero;
        NoteSymbol::Flat(UStepNonZero::try_from(u8::try_from($value).unwrap()).unwrap())
    }};

    () => { flat!(1) };
}

#[macro_export]
macro_rules! symbol {
    ($value:literal) => {{
        use $crate::note::relative::symbol::NoteSymbol;
        use $crate::step::UStepNonZero;
        match $value {
            0   => NoteSymbol::Natural,
            1.. => NoteSymbol::Sharp(UStepNonZero::try_from(u8::try_from($value).unwrap()).unwrap()),
            _   => NoteSymbol::Flat(UStepNonZero::try_from(u8::try_from(-$value).unwrap()).unwrap()),
        }
    }};

    () => { symbol!(0) };
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
