use std::fmt::Display;

pub enum Symbol {
    Sharp,
    Flat,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Symbol::Sharp => "♯",
            Symbol::Flat => "♭",
        })
    }
}
