pub mod pitch_class;
pub mod accidental;
pub mod octave;
pub mod note;
pub mod interval;
pub mod scale;

pub type Semitones = i8;

#[cfg(test)]
mod tests {
    use crate::{
        accidental::Accidental, interval::Interval, note::Note, octave::Octave,
        pitch_class::PitchClass,
    };

    #[test]
    fn cyclical_eq() {
        let pc = PitchClass::A;
        for i in 0..8 {
            assert_eq!(pc.next(i), pc.next(i + 7))
        }
    }

    #[test]
    fn to_flat() {
        let note = Note::empty()
            .set_accidental(Accidental(-8))
            .set_octave(Octave(4))
            .set_pitch_class(PitchClass::E)
            .to_flat();

        let dest = Note::empty()
            .set_accidental(Accidental(4))
            .set_octave(Octave(3))
            .set_pitch_class(PitchClass::E)
            .to_flat();

        assert_eq!(note, dest)
    }

    #[test]
    fn perfect_fifth() {
        let note = Note::empty()
            .set_pitch_class(PitchClass::C)
            .set_accidental(Accidental(1))
            .set_octave(Octave(4));

        println!("{:?}", note);

        let high = Note::empty()
            .set_pitch_class(PitchClass::G)
            .set_accidental(Accidental(1))
            .set_octave(Octave(4));

        assert_eq!(note + Interval::PERFECT_FIFTH, high);
    }

    #[test]
    fn perfect_fifth_desc() {
        let note = Note::empty()
            .set_accidental(Accidental(0))
            .set_pitch_class(PitchClass::C)
            .set_octave(Octave(4));

        let lowr = Note::empty()
            .set_accidental(Accidental(0))
            .set_pitch_class(PitchClass::F)
            .set_octave(Octave(3));

        assert_eq!(note - Interval::PERFECT_FIFTH, lowr);
    }

    #[test]
    fn perfect_fifth_accidentals() {
        for i in 0..6 {
            let note = Note::empty()
                .set_accidental(Accidental(i))
                .set_pitch_class(PitchClass::C)
                .set_octave(Octave(4))
                .to_flat();

            let high = Note::empty()
                .set_accidental(Accidental(i))
                .set_pitch_class(PitchClass::G)
                .set_octave(Octave(4))
                .to_flat();

            let lowr = Note::empty()
                .set_accidental(Accidental(i))
                .set_pitch_class(PitchClass::F)
                .set_octave(Octave(3))
                .to_flat();

            assert_eq!((note + Interval::PERFECT_FIFTH).to_flat(), high);
            assert_eq!((note - Interval::PERFECT_FIFTH).to_flat(), lowr);
        }
    }
}
