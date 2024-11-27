use music_theory_api::step::{IStep, IStepNonZero, NotNonZeroError, UStep, UStepNonZero};

#[test]
fn step_conversions() {
    // identity
    assert_eq!(UStep::from(1), UStep::from(1));
    assert_eq!(IStep::from(1), IStep::from(1));
    assert_eq!(
        UStepNonZero::try_from(1).unwrap(),
        UStepNonZero::try_from(1).unwrap()
    );
    assert_eq!(
        IStepNonZero::try_from(1).unwrap(),
        IStepNonZero::try_from(1).unwrap()
    );

    for i in u8::from(UStep::MIN)..=UStep::MAX.into() {
        assert_eq!(UStep::from(i), UStep::from(i));
    }
    
    for i in i8::from(IStep::MIN)..=IStep::MAX.into() {
        assert_eq!(IStep::from(i), IStep::from(i));
    }

    // check that 0 fails to parse
    for i in u8::from(UStepNonZero::MIN)..=UStepNonZero::MAX.into() {
        match i == 0 {
            false => assert_eq!(
                UStepNonZero::try_from(i).unwrap(),
                UStepNonZero::try_from(i).unwrap()
            ),
            true => assert_eq!(
                UStepNonZero::try_from(i),
                Err(NotNonZeroError)
            ),
        }
    }

    for i in i8::from(IStepNonZero::MIN)..=IStepNonZero::MAX.into() {
        match i == 0 {
            false => assert_eq!(
                IStepNonZero::try_from(i).unwrap(),
                IStepNonZero::try_from(i).unwrap()
            ),
            true => assert_eq!(
                IStepNonZero::try_from(i),
                Err(NotNonZeroError)
            ),
        }
    }
}
