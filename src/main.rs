use music_theory_api::{note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, symbol::NoteSymbol, RelativeNote}}, step::UStepNonZero};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::C,
        symbol: NoteSymbol::Flat(UStepNonZero::try_from(1).unwrap()),
    };
    println!("{}", note);
    println!("{}", AbsoluteNote::from(note));
}
