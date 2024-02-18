use crate::{unicode_block::UnicodeBlock, unicode_blocks::*};

const CJK_BLOCKS: [UnicodeBlock; 33] = [
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
    CJK_COMPATIBILITY,
    CJK_COMPATIBILITY_FORMS,
    CJK_COMPATIBILITY_IDEOGRAPHS,
    CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT,
    CJK_RADICALS_SUPPLEMENT, // 補充康熙部首不足的部份
    CJK_STROKES,
    CJK_SYMBOLS_AND_PUNCTUATION,
    HIRAGANA,
    KATAKANA,
    KATAKANA_PHONETIC_EXTENSIONS,
    KANA_EXTENDED_A,
    KANA_EXTENDED_B,
    KANA_SUPPLEMENT,
    HANGUL_JAMO,
    HANGUL_COMPATIBILITY_JAMO,
    HANGUL_JAMO_EXTENDED_A,
    HANGUL_JAMO_EXTENDED_B,
    HANGUL_SYLLABLES,
    HALFWIDTH_AND_FULLWIDTH_FORMS,      // ！, ，, ６, ＠, Ｚ, ﾔ
    ENCLOSED_CJK_LETTERS_AND_MONTHS,    // ㈠, ㋀
    ENCLOSED_IDEOGRAPHIC_SUPPLEMENT,    // 🈲, 🈧
    KANGXI_RADICALS,                    // 康熙部首
    IDEOGRAPHIC_DESCRIPTION_CHARACTERS, // ⿰, ⿸
];

/// Given a character, determine whether it is used in CJK.
#[inline]
pub fn is_cjk(c: char) -> bool {
    for b in CJK_BLOCKS.iter() {
        if b.contains(c) {
            return true;
        }
    }

    false
}

/// Given a `UnicodeBlock`, determine whether it is used in CJK.
#[inline]
pub fn is_cjk_block(unicode_block: UnicodeBlock) -> bool {
    for b in CJK_BLOCKS.iter().copied() {
        if unicode_block == b {
            return true;
        }
    }

    false
}
