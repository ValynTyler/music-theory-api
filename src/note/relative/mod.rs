use std::fmt::Display;

use letter::NoteLetter;
use symbol::NoteSymbol;

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
