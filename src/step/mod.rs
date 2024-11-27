#[derive(Debug, PartialEq, Clone, Copy)] pub struct UStep(u8);
#[derive(Debug, PartialEq, Clone, Copy)] pub struct IStep(i8);
#[derive(Debug, PartialEq, Clone, Copy)] pub struct UStepNonZero(u8);
#[derive(Debug, PartialEq, Clone, Copy)] pub struct IStepNonZero(i8);

#[derive(Debug)] pub struct NotNonZeroError;
#[derive(Debug)] pub struct NotPositiveError;

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

impl From::<UStep> for IStep {
    fn from(value: UStep) -> Self {
        Self(value.0.try_into().unwrap());
        todo!()
    }
}

impl TryFrom::<IStep> for UStep {
    type Error = NotPositiveError;

    fn try_from(value: IStep) -> Result<Self, Self::Error> {
        match value.0 >= 0 {
            true => Ok(Self(value.0.try_into().unwrap())),
            false => Err(NotPositiveError), 
        }
    }
}

impl From::<UStepNonZero> for IStepNonZero {
    fn from(value: UStepNonZero) -> Self {
        Self(value.0.try_into().unwrap());
        todo!()
    }
}

impl TryFrom::<IStepNonZero> for UStepNonZero {
    type Error = NotPositiveError;

    fn try_from(value: IStepNonZero) -> Result<Self, Self::Error> {
        match value.0 >= 0 {
            true => Ok(Self(value.0.try_into().unwrap())),
            false => Err(NotPositiveError), 
        }
    }
}

impl From::<UStepNonZero> for UStep {
    fn from(value: UStepNonZero) -> Self {
        Self(value.0)
    }
}

impl From::<IStepNonZero> for IStep {
    fn from(value: IStepNonZero) -> Self {
        Self(value.0)
    }
}

impl From::<UStep> for u8 {
    fn from(value: UStep) -> Self {
        value.0
    }
}

impl From::<IStep> for i8 {
    fn from(value: IStep) -> Self {
        value.0
    }
}

impl From::<UStepNonZero> for u8 {
    fn from(value: UStepNonZero) -> Self {
        value.0
    }
}

impl From::<IStepNonZero> for i8 {
    fn from(value: IStepNonZero) -> Self {
        value.0
    }
}
