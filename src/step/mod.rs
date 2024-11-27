pub struct UStep(u8);
pub struct IStep(i8);
pub struct UStepNonZero(u8);
pub struct IStepNonZero(i8);

pub struct NotNonZeroError;

impl From::<u8> for UStep {
    fn from(value: u8) -> Self {
        Self(value % 12)
    }
}

impl From::<i8> for IStep {
    fn from(value: i8) -> Self {
        Self(value % 12)
    }
}

impl TryFrom::<u8> for UStepNonZero {
    type Error = NotNonZeroError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = value % 12;
        match value {
            0 => Err(NotNonZeroError),
            _ => Ok(UStepNonZero(value)),
        }
    }
}

impl TryFrom::<i8> for IStepNonZero {
    type Error = NotNonZeroError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value = value % 12;
        match value {
            0 => Err(NotNonZeroError),
            _ => Ok(IStepNonZero(value)),
        }
    }
}
