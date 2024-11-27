use crate::step::UStep;

#[derive(Debug)]
pub struct AbsoluteNote(UStep);

impl From::<UStep> for AbsoluteNote {
    fn from(value: UStep) -> Self {
        Self(value)
    }
}
