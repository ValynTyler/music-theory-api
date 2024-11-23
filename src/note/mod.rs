#[derive(Debug)]
pub struct Note {
    pub pitch: u8,
}

impl Note {
    pub fn pitch(&self) -> u8 {
        self.pitch % 128
    }

    pub fn min() -> Self {
        Note { pitch: 0 }
    }

    pub fn max() -> Self {
        Note { pitch: 127 }
    }
}
