use crate::accidental::*;
use crate::pitch_class::letter::*;
use crate::*;

#[allow(unused)]
pub struct Note {
    pitch_class: PitchClass,
    octave: Octave,
    accidental: Accidental,
}

impl Note {
    // Constructor
    pub fn new(
        pitch_class: PitchClass,
        octave: Octave,
        accidental: Accidental
    ) -> Note {
        Note {
            pitch_class,
            octave,
            accidental,
        }
    }
}
