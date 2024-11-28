use music_theory_api::{note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, RelativeNote}}, symbol};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::B,
        symbol: symbol!(-1),
    };
    println!("{}", note);
    println!("{}", AbsoluteNote::from(note));
}
