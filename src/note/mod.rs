#[derive(Debug, PartialEq)]
pub struct Note {
    pub pitch: u8,
}

impl Note {
    pub const MAX: Note = Note { pitch: 127 };
    pub const MIN: Note = Note { pitch: 0 };

    pub fn pitch(&self) -> u8 {
        self.pitch % 128
    }
}

impl From::<u8> for Note {
    fn from(value: u8) -> Self {
        Note { pitch: value % 128 }
    }
}
