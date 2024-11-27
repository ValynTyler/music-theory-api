use music_theory_api::{istep, note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, symbol::NoteSymbol, RelativeNote}}, step::UStepNonZero, ustep, ustep_non_zero};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::C,
        symbol: NoteSymbol::Flat(ustep_non_zero!(1)),
    };
    println!("{}", note);
    println!("{}", AbsoluteNote::from(note));

    println!("{:?}", ustep!(2));
    println!("{:?}", istep!(2));
}
