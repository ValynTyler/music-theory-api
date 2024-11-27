use crate::step::UStep;

pub enum NoteLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl From::<NoteLetter> for UStep {
    fn from(value: NoteLetter) -> Self {
        match value {
            NoteLetter::C => UStep::from(0),
            NoteLetter::D => UStep::from(2),
            NoteLetter::E => UStep::from(4),
            NoteLetter::F => UStep::from(5),
            NoteLetter::G => UStep::from(7),
            NoteLetter::A => UStep::from(9),
            NoteLetter::B => UStep::from(11),
        }
    }
} 
