use std::ops;

use crate::{accidental::Accidental, note::Note, octave::Octave, Semitones};

pub struct Interval {
    steps: u8,
    semitones: Semitones,
}

impl Interval {
    pub const MINOR_THIRD: Interval = Interval {
        steps: 2,
        semitones: 3,
    };

    pub const MAJOR_THIRD: Interval = Interval {
        steps: 2,
        semitones: 4,
    };

    pub const PERFECT_FIFTH: Interval = Interval {
        steps: 4,
        semitones: 7,
    };

    pub fn new(steps: u8, semitones: Semitones) -> Interval {
        Interval { steps, semitones }
    }

    pub fn apply(&self, note: Note) -> Note {
        let mut note: Note = note;
        
        let delta = self.semitones;
        let _start_semitones = note.semitones();
        let _final_semitones = note.semitones() + delta;

        let octave = Octave(_final_semitones / 12);
        let pitch_class = match delta > 0 {
            true => note.pitch_class.next(self.steps),
            false => note.pitch_class.prev(self.steps),
        };
        let accidental = note.accidental + Accidental(
            _final_semitones -
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
