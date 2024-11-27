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

struct Note {
    letter: NoteLetter,
    symbol: NoteSymbol,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.letter, self.symbol)
    }
}

fn main() {
    let note = Note {
        letter: NoteLetter::G,
        symbol: NoteSymbol::Sharp(1.try_into().unwrap()),
    };
    println!("{}", note);
}
