use music_theory_api::{note::relative::{letter::NoteLetter, symbol::NoteSymbol, RelativeNote}, step::UStepNonZero};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::A,
        symbol: NoteSymbol::Flat(UStepNonZero::try_from(3).unwrap()),
    };
    println!("{}", note);
}
