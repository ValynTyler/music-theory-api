struct NotePitch(u8); // between 0 and 11

impl From::<u8> for NotePitch {
    fn from(value: u8) -> Self {
        Self(value % 12)
    }
}

impl From<NoteLetter> for NotePitch {
    fn from(value: NoteLetter) -> Self {
        match value {
            NoteLetter::C => 0.into(),
            NoteLetter::D => 2.into(),
            NoteLetter::E => 4.into(),
            NoteLetter::F => 5.into(),
            NoteLetter::G => 7.into(),
            NoteLetter::A => 9.into(),
            NoteLetter::B => 11.into(),
        }
    }
}

enum NoteLetter {
    A, B, C, D, E, F, G
}

enum NoteSymbol {
    Flat(NotePitch), // TODO doesnt make sense to represent `Flat(0)`
    Sharp(NotePitch),
    Natural,
}


fn main() {
    println!("{}", -13 % 122);
}
