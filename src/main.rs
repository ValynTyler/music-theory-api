use std::fmt::Display;

#[derive(Debug)]
enum NoteLetter {
    A, B, C, D, E, F, G
}

impl Display for NoteLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum NoteSymbol {
    Sharp(u8), Flat(u8)
}

#[derive(Debug)]
struct Note {
    letter: NoteLetter,
    symbol: NoteSymbol,
}

fn main() {
    println!("{:?}", Note { letter: NoteLetter::A, symbol: NoteSymbol::Sharp(1) });
}
