use core::fmt;

use crate::{interval::Interval, note::Note};

#[derive(Debug)]
pub struct Scale(pub Vec<Note>);

impl Scale {
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
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scale[").unwrap();
        for i in 0..self.0.len()-1 {
            write!(f, "{}, ", self.0[i]).unwrap();
        }
        write!(f, "{}", self.0[self.0.len()-1]).unwrap(); // TODO: I should definitely do error handling
        write!(f, "]").unwrap();
        write!(f, "")
    }
}
