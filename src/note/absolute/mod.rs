use std::fmt::Display;

use crate::step::UStep;

pub struct AbsoluteNote(UStep);

impl From::<UStep> for AbsoluteNote {
    fn from(value: UStep) -> Self {
        Self(value)
    }
}

impl Display for AbsoluteNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(self.0))
    }
}
