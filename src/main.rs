use music_theory_api::{note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, RelativeNote}}, sharp, symbol};

macro_rules! count_symbols {
    () => {
        0
    };

    (# $($rest:tt)*) => {
        count_symbols!($($rest)*) + 1
    };

    (b $($rest:tt)*) => {
        count_symbols!($($rest)*) - 1
    };
}

macro_rules! note_letter {
    (C) => { NoteLetter::C };
    (D) => { NoteLetter::D };
    (E) => { NoteLetter::E };
    (F) => { NoteLetter::F };
    (G) => { NoteLetter::G };
    (A) => { NoteLetter::A };
    (B) => { NoteLetter::B };
}

macro_rules! note {
    ($letter:ident $($symbols:tt)*) => {
        RelativeNote {
            letter: note_letter!($letter),
            symbol: symbol!(count_symbols!($($symbols)*)),
        }
    };
}

fn main() {
    let scale = vec![
        RelativeNote { letter: NoteLetter::C, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::D, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::E, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::F, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::G, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::A, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::B, symbol: symbol!() },
    ];

    let chromatic = vec![
        RelativeNote { letter: NoteLetter::C, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::C, symbol: sharp!() },
        RelativeNote { letter: NoteLetter::D, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::D, symbol: sharp!() },
        RelativeNote { letter: NoteLetter::E, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::F, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::F, symbol: sharp!() },
        RelativeNote { letter: NoteLetter::G, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::G, symbol: sharp!() },
        RelativeNote { letter: NoteLetter::A, symbol: symbol!() },
        RelativeNote { letter: NoteLetter::A, symbol: sharp!() },
        RelativeNote { letter: NoteLetter::B, symbol: symbol!() },
    ];

    for note in scale {
        println!("{} {}", note, AbsoluteNote::from(note))
    }
    println!();

    for note in chromatic {
        println!("{:<2} {}", note.to_string(), AbsoluteNote::from(note))
    }
    println!();

    let jeff = note!(G b b);
    println!("{}", jeff);
}
