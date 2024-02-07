use crate::accidental::Accidental;
use crate::pitch_class::letter::*;
use crate::Octave;

#[allow(unused)]
#[derive(Debug)]
pub struct Note {
    pitch_class: PitchClass,
    accidental: Accidental,
    octave: Octave,
}

impl Note {
    /// ## Constructor
    /// Creates a new instance of note, given the `PitchClass`, `Accidental` and `Octave`.
    pub fn new(
        pitch_class: PitchClass,
        accidental: Accidental,
        octave: Octave,
    ) -> Note {
        Note {
            pitch_class,
            accidental,
            octave,
        }
    }
}
