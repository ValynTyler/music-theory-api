use music_theory_api::{
    note::Note, pitch_class::PitchClass, scale::Scale
};

fn main() {
    let note = Note::empty()
        .set_pitch_class(PitchClass::A);

    let key: Scale = Scale::minor_triad(note);

    println!("{}", key);
}
