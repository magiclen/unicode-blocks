use crate::{unicode_block::UnicodeBlock, unicode_blocks::*};

const CJK_BLOCKS: [UnicodeBlock; 45] = [
    CJK_UNIFIED_IDEOGRAPHS,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_I,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_J,
    CJK_COMPATIBILITY,
    CJK_COMPATIBILITY_FORMS,
    CJK_COMPATIBILITY_IDEOGRAPHS,
    CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT,
    CJK_RADICALS_SUPPLEMENT, // supplements the Kangxi radicals
    CJK_STROKES,
    CJK_SYMBOLS_AND_PUNCTUATION,
    IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION,
    YIJING_HEXAGRAM_SYMBOLS,
    HIRAGANA,
    KATAKANA,
    KATAKANA_PHONETIC_EXTENSIONS,
    KANA_EXTENDED_A,
    KANA_EXTENDED_B,
    KANA_SUPPLEMENT,
    SMALL_KANA_EXTENSION,
    KANBUN,
    BOPOMOFO,
    BOPOMOFO_EXTENDED,
    HANGUL_JAMO,
    HANGUL_COMPATIBILITY_JAMO,
    HANGUL_JAMO_EXTENDED_A,
    HANGUL_JAMO_EXTENDED_B,
    HANGUL_SYLLABLES,
    HALFWIDTH_AND_FULLWIDTH_FORMS, // ！, ，, ６, ＠, Ｚ, ﾔ
    VERTICAL_FORMS,
    SMALL_FORM_VARIANTS,
    ENCLOSED_CJK_LETTERS_AND_MONTHS, // ㈠, ㋀
    ENCLOSED_IDEOGRAPHIC_SUPPLEMENT, // 🈲, 🈧
    KANGXI_RADICALS,
    IDEOGRAPHIC_DESCRIPTION_CHARACTERS, // ⿰, ⿸
    SEAL,
    TAI_XUAN_JING_SYMBOLS,
    NUSHU,
    // COUNTING_ROD_NUMERALS is not included because it also has Western tally marks (U+1D377, U+1D378).
];

const CJK_MIN_START: u32 = {
    let mut min = u32::MAX;
    let mut i = 0;

    while i < CJK_BLOCKS.len() {
        if CJK_BLOCKS[i].start < min {
            min = CJK_BLOCKS[i].start;
        }

        i += 1;
    }

    min
};

/// Returns whether the character belongs to a block in the CJK set.
///
/// See [CJK blocks](crate#cjk-blocks) for the scope of this set.
#[inline]
pub const fn is_cjk(c: char) -> bool {
    // Characters below every CJK block, such as ASCII, can skip the scan.
    if (c as u32) < CJK_MIN_START {
        return false;
    }

    let mut i = 0;

    while i < CJK_BLOCKS.len() {
        if CJK_BLOCKS[i].contains(c) {
            return true;
        }

        i += 1;
    }

    false
}

/// Returns whether the Unicode block is in the CJK set.
///
/// See [CJK blocks](crate#cjk-blocks) for the scope of this set.
#[inline]
pub const fn is_cjk_block(unicode_block: UnicodeBlock) -> bool {
    let mut i = 0;

    while i < CJK_BLOCKS.len() {
        // `==` on `UnicodeBlock` is not const, so compare `start` the same way `UnicodeBlock::eq` does.
        if CJK_BLOCKS[i].start == unicode_block.start {
            return true;
        }

        i += 1;
    }

    false
}
