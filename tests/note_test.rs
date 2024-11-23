use music_theory_api::note::Note;

#[test]
fn test_midi_parity() {
    assert_eq!(0x00 as u8, Note::min().pitch());
    assert_eq!(0x7F as u8, Note::max().pitch());

    for i in 0..u8::MAX / 2 + 1 {
        assert_eq!(Note::from(i), Note::from(i));
        assert_eq!(Note::from(i).pitch(), Note::from(i + 128).pitch());
    }
}
