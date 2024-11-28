use music_theory_api::{istep, note::{absolute::AbsoluteNote, relative::{letter::NoteLetter, RelativeNote}}, sharp, symbol, ustep};

fn main() {
    let note = RelativeNote {
        letter: NoteLetter::B,
        symbol: symbol!(2),
    };
    println!("{}", note);
    println!("{}", AbsoluteNote::from(note));

    println!("{:?}", ustep!(2));
    println!("{:?}", istep!(2));
}
