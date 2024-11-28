use music_theory_api::{note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, RelativeNote}}, sharp, symbol};

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
}
