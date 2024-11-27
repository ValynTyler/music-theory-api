use letter::NoteLetter;
use symbol::NoteSymbol;

pub mod letter;
pub mod symbol;

pub struct RelativeNote {
    pub letter: NoteLetter,
    pub symbol: NoteSymbol,
}
