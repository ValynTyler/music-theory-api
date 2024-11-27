use music_theory_api::{note::relative::{letter::NoteLetter, symbol::NoteSymbol, RelativeNote}, step::UStepNonZero};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::C,
        symbol: NoteSymbol::Natural,
    };
    println!("{}", NoteSymbol::Flat(UStepNonZero::try_from(7).unwrap()));
}
