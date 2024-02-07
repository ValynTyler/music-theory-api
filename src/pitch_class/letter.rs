#[derive(Copy, Clone, Debug)]
pub enum PitchClass {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl PitchClass {
    pub fn semitones(&self) -> i8 { // TODO: SHOULD NOT RETURN i8
        match self {
            PitchClass::C => 0,
            PitchClass::D => 2,
            PitchClass::E => 4,
            PitchClass::F => 5,
            PitchClass::G => 7,
            PitchClass::A => 9,
            PitchClass::B => 11,
        }
    }
}