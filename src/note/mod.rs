use core::fmt;
use std::ops;

use crate::accidental::Accidental;
use crate::interval::Interval;
use crate::{pitch_class::*, Semitones};
use crate::octave::Octave;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub accidental: Accidental,
    pub octave: Octave,
}

impl Note {
    pub const MIDDLE_C: Note = Note::new(PitchClass::C, Accidental(0), Octave(4));

    pub const fn new(pitch_class: PitchClass, accidental: Accidental, octave: Octave) -> Note {
        Note {
            pitch_class,
            accidental,
            octave,
        }
    }

    pub const fn empty() -> Note {
        Note {
            pitch_class: PitchClass::C,
            accidental: Accidental(0),
            octave: Octave(0),
        }
    }

    pub fn semitones(&self) -> Semitones {
        let pcl_semitones = self.pitch_class.semitones();
        let acc_semitones = self.accidental.semitones();
        let oct_semitones = self.octave.semitones();
    
        pcl_semitones + acc_semitones + oct_semitones
    }

    pub fn set_pitch_class(&mut self, value: PitchClass) -> Note {
        self.pitch_class = value;
        *self
    }

    pub fn set_accidental(&mut self, value: Accidental) -> Note {
        self.accidental = value;
        *self
    }

    pub fn set_octave(&mut self, value: Octave) -> Note {
        self.octave = value;
        *self
    }

    pub fn to_flat(&mut self) -> Note {
        if self.accidental.is_sharp() {
            self.set_octave(self.octave + 1);
            self.set_accidental(self.accidental - Accidental(12));   
        }
        *self
    }
}

impl ops::Add<Interval> for Note {
    type Output = Note;

    fn add(self, rhs: Interval) -> Self::Output {
        rhs.apply(self)
    }
}

impl ops::Sub<Interval> for Note {
    type Output = Note;

    fn sub(self, rhs: Interval) -> Self::Output {
        (-rhs).apply(self)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class, self.accidental)
    }
}
