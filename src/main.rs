pub struct AbsoluteNote {
    pitch: u8, // between 8 and 127
}

pub struct RelativeNote {
    base: u8,           // between 0 and 11
    symbol: i8,         // any value
    pitch_class: u8,    // between 0 and 9
                        // converted absolute value will be between 0 and 9 * 12 + 11 (119)
}

fn main() {
}
