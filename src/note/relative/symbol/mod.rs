use crate::step::UStep;

pub enum NoteSymbol {
    Sharp(UStep),
    Flat(UStep),
    Natural,
}
