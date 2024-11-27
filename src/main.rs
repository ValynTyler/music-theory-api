use music_theory_api::{istep, note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, symbol::NoteSymbol, RelativeNote}}, step::UStepNonZero, ustep};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::C,
        symbol: NoteSymbol::Flat(UStepNonZero::try_from(1).unwrap()),
    };
    println!("{}", note);
    println!("{}", AbsoluteNote::from(note));

    println!("{:?}", ustep!(2));
    println!("{:?}", istep!(2));
}
