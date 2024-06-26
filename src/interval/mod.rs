use std::ops;

use crate::{accidental::Accidental, note::Note, octave::Octave, Semitones};

pub struct Interval {
    steps: u8,
    semitones: Semitones,
}

impl Interval {
    pub const MINOR_SECOND: Interval = Interval {
        steps: 1,
        semitones: 1,
    };
    
    pub const MAJOR_SECOND: Interval = Interval {
        steps: 1,
        semitones: 2,
    };

    pub const MINOR_THIRD: Interval = Interval {
        steps: 2,
        semitones: 3,
    };

    pub const MAJOR_THIRD: Interval = Interval {
        steps: 2,
        semitones: 4,
    };
    
    pub const PERFECT_FOURTH: Interval = Interval {
        steps: 3,
        semitones: 5,
    };

    pub const AUGMENTED_FOURTH: Interval = Interval {
        steps: 3,
        semitones: 6,
    };

    pub const DIMINISHED_FIFTH: Interval = Interval {
        steps: 4,
        semitones: 6,
    };

    pub const PERFECT_FIFTH: Interval = Interval {
        steps: 4,
        semitones: 7,
    };

    pub const MINOR_SIXTH: Interval = Interval {
        steps: 5,
        semitones: 8,
    };
    
    pub const MAJOR_SIXTH: Interval = Interval {
        steps: 5,
        semitones: 9,
    };

    pub const MINOR_SEVENTH: Interval = Interval {
        steps: 6,
        semitones: 10,
    };
    
    pub const MAJOR_SEVENTH: Interval = Interval {
        steps: 6,
        semitones: 11,
    };

    pub const PERFECT_OCTAVE: Interval = Interval {
        steps: 7,
        semitones: 12,
    };

    pub fn new(steps: u8, semitones: Semitones) -> Interval {
        Interval { steps, semitones }
    }

    pub fn apply(&self, note: Note) -> Note {
        let mut note: Note = note;
        
        let delta = self.semitones;
        let start_semitones = note.semitones();
        let final_semitones = start_semitones + delta;

        let octave = Octave(final_semitones / 12);
        let pitch_class = match delta > 0 {
            true => note.pitch_class.next(self.steps),
            false => note.pitch_class.prev(self.steps),
        };
        let accidental = note.accidental + Accidental(
            final_semitones -
            note.set_pitch_class(pitch_class)
                .set_octave(octave)
                .semitones()
        );

        note.set_octave(octave)
            .set_accidental(accidental)
            .set_pitch_class(pitch_class)
    }
}

impl ops::Neg for Interval {
    type Output = Interval;

    fn neg(self) -> Self::Output {
        Interval {
            steps: self.steps,
            semitones: -self.semitones,
        }
    }
}
