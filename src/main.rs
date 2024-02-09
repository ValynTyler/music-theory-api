use music_theory_api::{
    accidental::Accidental, note::Note, octave::Octave, pitch_class::PitchClass::{self}
};

fn main() {
    const STRINGS: usize = 6;
    const FRETS: i8 = 18;

    let tuning = [
        Note::new(PitchClass::E, Accidental::NATURAL, Octave(4)),
        Note::new(PitchClass::B, Accidental::NATURAL, Octave(3)),
        Note::new(PitchClass::G, Accidental::NATURAL, Octave(3)),
        Note::new(PitchClass::D, Accidental::NATURAL, Octave(3)),
        Note::new(PitchClass::A, Accidental::NATURAL, Octave(2)),
        Note::new(PitchClass::E, Accidental::NATURAL, Octave(2)),
    ];

    let key = [
        Note::new(PitchClass::C, Accidental::NATURAL, Octave(0)),
        Note::new(PitchClass::E, Accidental(-1), Octave(0)),
        Note::new(PitchClass::G, Accidental::NATURAL, Octave(0)),
        Note::new(PitchClass::B, Accidental::NATURAL, Octave(0)),
        Note::new(PitchClass::B, Accidental::NATURAL, Octave(0)),
        Note::new(PitchClass::D, Accidental::NATURAL, Octave(0)),
        Note::new(PitchClass::E, Accidental::NATURAL, Octave(0)),
        Note::new(PitchClass::F, Accidental::NATURAL, Octave(0)),
    ];

    for i in 0..STRINGS {
        for j in 0..FRETS {
            if j == 1 { print!("|"); }
            if j != 0 { print!("|"); }

            let idx = tuning[i].semitones() + j;
            let mut s = String::new();
            for k in key {
                if idx % 12 == k.semitones() {
                    s = k.to_string();
                }
            }

            let indent = 5;
            print!("-{:-<indent$}-", s)
        }
        println!("");
    }
}
