#[derive(Debug)]
pub struct Note {
    pub pitch: i8,
}

impl Note {
    pub fn pitch(&self) -> i8 {
        // to conform with MIDI, notes are constrained between 0 and 127
        self.pitch.abs()
    }
}
