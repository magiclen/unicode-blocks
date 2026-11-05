#[test]
fn find_unicode_18_blocks() {
    for (block, name, start, end) in [
        (unicode_blocks::BENGALI_SUPPLEMENT, "Bengali Supplement", 0x11DF0, 0x11DFF),
        (
            unicode_blocks::ARCHAIC_CUNEIFORM_NUMERALS,
            "Archaic Cuneiform Numerals",
            0x12550,
            0x1268F,
        ),
        (unicode_blocks::JURCHEN, "Jurchen", 0x18E00, 0x1919F),
        (unicode_blocks::JURCHEN_RADICALS, "Jurchen Radicals", 0x191A0, 0x191DF),
        (
            unicode_blocks::MUSICAL_SYMBOLS_SUPPLEMENT,
            "Musical Symbols Supplement",
            0x1D250,
            0x1D28F,
        ),
        (
            unicode_blocks::MISCELLANEOUS_SYMBOLS_AND_ARROWS_EXTENDED,
            "Miscellaneous Symbols and Arrows Extended",
            0x1DB00,
            0x1DBFF,
        ),
        (unicode_blocks::SEAL, "Seal", 0x3D000, 0x3FC3F),
    ] {
        assert_eq!(name, block.name());
        assert_eq!(start, block.start());
        assert_eq!(end, block.end());

        for code_point in [start, end] {
            assert_eq!(
                Some(block),
                unicode_blocks::find_unicode_block(char::from_u32(code_point).unwrap())
            );
        }
    }
}
