#[derive(Debug)]
pub struct Accidental(pub i8);

impl Accidental {
    pub const NATURAL: Accidental = Accidental(0);
    pub const SHARP: Accidental = Accidental(1);
    pub const FLAT: Accidental = Accidental(-1);

    pub const BECAR: Accidental = Accidental(0);
    pub const DIEZ: Accidental = Accidental(1);
    pub const BEMOL: Accidental = Accidental(-1);
}
