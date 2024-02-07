pub mod note;
pub mod pitch_class;
pub mod accidental;
pub mod interval;

pub type Semitones = i8;

#[derive(Debug)]
pub struct Octave(pub i8);
