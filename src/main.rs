use std::fmt::Display;

struct NotePitch(u8); // between 0 and 11

impl From::<u8> for NotePitch {
    fn from(value: u8) -> Self {
        Self(value % 12)
    }
}

impl From<NoteLetter> for NotePitch {
    fn from(value: NoteLetter) -> Self {
        match value {
            NoteLetter::C => 0.into(),
            NoteLetter::D => 2.into(),
            NoteLetter::E => 4.into(),
            NoteLetter::F => 5.into(),
            NoteLetter::G => 7.into(),
            NoteLetter::A => 9.into(),
            NoteLetter::B => 11.into(),
        }
    }
}

struct NoteShift(u8);
#[derive(Debug)]
struct ZeroShiftError;

impl TryFrom::<u8> for NoteShift {
    type Error = ZeroShiftError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value % 12 {
            0 => Err(ZeroShiftError),
            _ => Ok(Self(value % 12))
        }
    }
}

#[derive(Debug)]
enum NoteLetter {
    A, B, C, D, E, F, G
}

enum NoteSymbol {
    Flat(NoteShift),
    Sharp(NoteShift),
    Natural,
}

impl Display for NoteSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteSymbol::Sharp(note_shift) => {
                for _ in 0..note_shift.0 {
                    write!(f, "♯")? // TODO implement double sharp(𝄪) display
                }
            }
            NoteSymbol::Flat(note_shift) => {
                for _ in 0..note_shift.0 {
                    write!(f, "♭")?
                }
            },
            NoteSymbol::Natural => write!(f, "♮")?,
        };
        Ok(())
    }
}

struct RelativeNote {
    letter: NoteLetter,
    symbol: NoteSymbol,
}

impl Display for RelativeNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.letter, self.symbol)
    }
}

struct AbsoluteNote(NotePitch);

impl From<RelativeNote> for AbsoluteNote {
    fn from(value: RelativeNote) -> Self {
        let letter_value: NotePitch = value.letter.into();
        let letter_value: i8 = letter_value.0 as i8;
        let symbol_value: i8 = match value.symbol {
            NoteSymbol::Flat(note_shift) => -(note_shift.0 as i8),
            NoteSymbol::Sharp(note_shift) => note_shift.0 as i8,
            NoteSymbol::Natural => 0,
        };
        let result = (24 + letter_value + symbol_value) % 12;
        Self(NotePitch(result as u8))
    }
}

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::B,
        symbol: NoteSymbol::Flat(1.try_into().unwrap()),
    };
    println!("{}", note);
    println!("{}", AbsoluteNote::from(note).0.0);
}
