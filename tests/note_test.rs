use music_theory_api::note::Note;

#[test]
fn test_midi_parity() {
    assert_eq!(0x00 as u8, Note::min().pitch());
    assert_eq!(0x7F as u8, Note::max().pitch());
}
