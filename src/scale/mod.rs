use core::fmt;

use crate::{interval::Interval, note::Note};

pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

pub enum ScaleType {
    Minor,
    Major,
    MinorPentatonic,
    MajorPentatonic,
    Modal(Mode),
}

#[derive(Debug)]
pub struct Scale(pub Vec<Note>);

impl Scale {
    pub fn build(root_note: Note, scale_type: ScaleType) -> Scale {
        match scale_type {
            ScaleType::Major => Scale(vec![
                root_note,
                root_note + Interval::MAJOR_SECOND,
                root_note + Interval::MAJOR_THIRD,
                root_note + Interval::PERFECT_FOURTH,
                root_note + Interval::PERFECT_FIFTH,
                root_note + Interval::MAJOR_SIXTH,
                root_note + Interval::MAJOR_SEVENTH,
            ]),
            ScaleType::Minor => Scale(vec![
                root_note,
                root_note + Interval::MAJOR_SECOND,
                root_note + Interval::MINOR_THIRD,
                root_note + Interval::PERFECT_FOURTH,
                root_note + Interval::PERFECT_FIFTH,
                root_note + Interval::MINOR_SIXTH,
                root_note + Interval::MINOR_SEVENTH,
            ]),
            ScaleType::MajorPentatonic => Scale(vec![
                root_note,
                root_note + Interval::MAJOR_SECOND,
                root_note + Interval::MAJOR_THIRD,
                root_note + Interval::PERFECT_FIFTH,
                root_note + Interval::MAJOR_SIXTH,
            ]),
            ScaleType::MinorPentatonic => Scale(vec![
                root_note,
                root_note + Interval::MINOR_THIRD,
                root_note + Interval::PERFECT_FOURTH,
                root_note + Interval::PERFECT_FIFTH,
                root_note + Interval::MINOR_SEVENTH,
            ]),
            ScaleType::Modal(mode) => match mode {
                Mode::Ionian => Scale(vec![
                    root_note,
                    root_note + Interval::MAJOR_SECOND,
                    root_note + Interval::MAJOR_THIRD,
                    root_note + Interval::PERFECT_FOURTH,
                    root_note + Interval::PERFECT_FIFTH,
                    root_note + Interval::MAJOR_SIXTH,
                    root_note + Interval::MAJOR_SEVENTH,
                ]),
                Mode::Dorian => Scale(vec![
                    root_note,
                    root_note + Interval::MAJOR_SECOND,
                    root_note + Interval::MINOR_THIRD,
                    root_note + Interval::PERFECT_FOURTH,
                    root_note + Interval::PERFECT_FIFTH,
                    root_note + Interval::MAJOR_SIXTH,
                    root_note + Interval::MINOR_SEVENTH,
                ]),
                Mode::Phrygian => Scale(vec![
                    root_note,
                    root_note + Interval::MINOR_SECOND,
                    root_note + Interval::MINOR_THIRD,
                    root_note + Interval::PERFECT_FOURTH,
                    root_note + Interval::PERFECT_FIFTH,
                    root_note + Interval::MINOR_SIXTH,
                    root_note + Interval::MINOR_SEVENTH,
                ]),
                Mode::Lydian => Scale(vec![
                    root_note,
                    root_note + Interval::MAJOR_SECOND,
                    root_note + Interval::MAJOR_THIRD,
                    root_note + Interval::AUGMENTED_FOURTH,
                    root_note + Interval::PERFECT_FIFTH,
                    root_note + Interval::MAJOR_SIXTH,
                    root_note + Interval::MAJOR_SEVENTH,
                ]),
                Mode::Mixolydian => Scale(vec![
                    root_note,
                    root_note + Interval::MAJOR_SECOND,
                    root_note + Interval::MAJOR_THIRD,
                    root_note + Interval::PERFECT_FOURTH,
                    root_note + Interval::PERFECT_FIFTH,
                    root_note + Interval::MAJOR_SIXTH,
                    root_note + Interval::MINOR_SEVENTH,
                ]),
                Mode::Aeolian => Scale(vec![
                    root_note,
                    root_note + Interval::MAJOR_SECOND,
                    root_note + Interval::MINOR_THIRD,
                    root_note + Interval::PERFECT_FOURTH,
                    root_note + Interval::PERFECT_FIFTH,
                    root_note + Interval::MINOR_SIXTH,
                    root_note + Interval::MINOR_SEVENTH,
                ]),
                Mode::Locrian => Scale(vec![
                    root_note,
                    root_note + Interval::MINOR_SECOND,
                    root_note + Interval::MINOR_THIRD,
                    root_note + Interval::PERFECT_FOURTH,
                    root_note + Interval::DIMINISHED_FIFTH,
                    root_note + Interval::MINOR_SIXTH,
                    root_note + Interval::MINOR_SEVENTH,
                ]),
            },
        }
    }

    pub fn major_triad(root_note: Note) -> Scale {
        Scale(vec![
            root_note,
            root_note + Interval::MAJOR_THIRD,
            root_note + Interval::PERFECT_FIFTH,
        ])
    }

    pub fn minor_triad(root_note: Note) -> Scale {
        Scale(vec![
            root_note,
            root_note + Interval::MINOR_THIRD,
            root_note + Interval::PERFECT_FIFTH,
        ])
    }

    pub fn to_sharp(mut self) -> Scale {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i].to_sharp()
        }
        self
    }

    pub fn to_flat(mut self) -> Scale {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i].to_flat()
        }
        self
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scale[").unwrap();
        for i in 0..self.0.len() - 1 {
            write!(f, "{}, ", self.0[i]).unwrap();
        }
        write!(f, "{}", self.0[self.0.len() - 1]).unwrap(); // TODO: I should definitely do error handling
        write!(f, "]").unwrap();
        write!(f, "")
    }
}
