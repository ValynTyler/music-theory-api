use std::fmt::Display;

use letter::NoteLetter;
use symbol::NoteSymbol;

use crate::step::{IStep, UStep};

use super::absolute::AbsoluteNote;

pub mod letter;
pub mod symbol;

pub struct RelativeNote {
    pub letter: NoteLetter,
    pub symbol: NoteSymbol,
}

impl Display for RelativeNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.symbol)
    }
}

impl From<RelativeNote> for AbsoluteNote {
    fn from(value: RelativeNote) -> Self {
        let ustep: UStep = value.letter.into();
        let istep: IStep = match value.symbol {
            NoteSymbol::Sharp(ustep_non_zero) => IStep::from(UStep::from(ustep_non_zero)),
            NoteSymbol::Flat(ustep_non_zero) => -IStep::from(UStep::from(ustep_non_zero)),
            NoteSymbol::Natural => IStep::from(0),
        };

        let result = u8::from(ustep) as i8 + i8::from(istep);
        let result = (result + 12) % 12;
        let result = result as u8;

        Self::from(UStep::from(result))
    }
}
