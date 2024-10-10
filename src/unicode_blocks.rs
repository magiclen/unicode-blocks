// The dataset is from https://www.unicode.org/Public/UNIDATA/Blocks.txt

use crate::UnicodeBlock;

pub const VERSION: &str = "16.0.0";
pub const BASIC_LATIN: UnicodeBlock =
    UnicodeBlock {
        name: "Basic Latin", start: 0x0000, end: 0x007F
    };
pub const LATIN_1_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Latin-1 Supplement", start: 0x0080, end: 0x00FF
    };
pub const LATIN_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-A", start: 0x0100, end: 0x017F
    };
pub const LATIN_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-B", start: 0x0180, end: 0x024F
    };
pub const IPA_EXTENSIONS: UnicodeBlock =
    UnicodeBlock {
        name: "IPA Extensions", start: 0x0250, end: 0x02AF
    };
pub const SPACING_MODIFIER_LETTERS: UnicodeBlock =
    UnicodeBlock {
        name: "Spacing Modifier Letters", start: 0x02B0, end: 0x02FF
    };
pub const COMBINING_DIACRITICAL_MARKS: UnicodeBlock =
    UnicodeBlock {
        name: "Combining Diacritical Marks", start: 0x0300, end: 0x036F
    };
pub const GREEK_AND_COPTIC: UnicodeBlock =
    UnicodeBlock {
        name: "Greek and Coptic", start: 0x0370, end: 0x03FF
    };
pub const CYRILLIC: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic", start: 0x0400, end: 0x04FF
};
pub const CYRILLIC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Cyrillic Supplement", start: 0x0500, end: 0x052F
    };
pub const ARMENIAN: UnicodeBlock = UnicodeBlock {
    name: "Armenian", start: 0x0530, end: 0x058F
};
pub const HEBREW: UnicodeBlock = UnicodeBlock {
    name: "Hebrew", start: 0x0590, end: 0x05FF
};
pub const ARABIC: UnicodeBlock = UnicodeBlock {
    name: "Arabic", start: 0x0600, end: 0x06FF
};
pub const SYRIAC: UnicodeBlock = UnicodeBlock {
    name: "Syriac", start: 0x0700, end: 0x074F
};
pub const ARABIC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Arabic Supplement", start: 0x0750, end: 0x077F
    };
pub const THAANA: UnicodeBlock = UnicodeBlock {
    name: "Thaana", start: 0x0780, end: 0x07BF
};
pub const NKO: UnicodeBlock = UnicodeBlock {
    name: "NKo", start: 0x07C0, end: 0x07FF
};
pub const SAMARITAN: UnicodeBlock =
    UnicodeBlock {
        name: "Samaritan", start: 0x0800, end: 0x083F
    };
pub const MANDAIC: UnicodeBlock = UnicodeBlock {
    name: "Mandaic", start: 0x0840, end: 0x085F
};
pub const SYRIAC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Syriac Supplement", start: 0x0860, end: 0x086F
    };
pub const ARABIC_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Arabic Extended-B", start: 0x0870, end: 0x089F
    };
pub const ARABIC_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Arabic Extended-A", start: 0x08A0, end: 0x08FF
    };
pub const DEVANAGARI: UnicodeBlock =
    UnicodeBlock {
        name: "Devanagari", start: 0x0900, end: 0x097F
    };
pub const BENGALI: UnicodeBlock = UnicodeBlock {
    name: "Bengali", start: 0x0980, end: 0x09FF
};
pub const GURMUKHI: UnicodeBlock = UnicodeBlock {
    name: "Gurmukhi", start: 0x0A00, end: 0x0A7F
};
pub const GUJARATI: UnicodeBlock = UnicodeBlock {
    name: "Gujarati", start: 0x0A80, end: 0x0AFF
};
pub const ORIYA: UnicodeBlock = UnicodeBlock {
    name: "Oriya", start: 0x0B00, end: 0x0B7F
};
pub const TAMIL: UnicodeBlock = UnicodeBlock {
    name: "Tamil", start: 0x0B80, end: 0x0BFF
};
pub const TELUGU: UnicodeBlock = UnicodeBlock {
    name: "Telugu", start: 0x0C00, end: 0x0C7F
};
pub const KANNADA: UnicodeBlock = UnicodeBlock {
    name: "Kannada", start: 0x0C80, end: 0x0CFF
};
pub const MALAYALAM: UnicodeBlock =
    UnicodeBlock {
        name: "Malayalam", start: 0x0D00, end: 0x0D7F
    };
pub const SINHALA: UnicodeBlock = UnicodeBlock {
    name: "Sinhala", start: 0x0D80, end: 0x0DFF
};
pub const THAI: UnicodeBlock = UnicodeBlock {
    name: "Thai", start: 0x0E00, end: 0x0E7F
};
pub const LAO: UnicodeBlock = UnicodeBlock {
    name: "Lao", start: 0x0E80, end: 0x0EFF
};
pub const TIBETAN: UnicodeBlock = UnicodeBlock {
    name: "Tibetan", start: 0x0F00, end: 0x0FFF
};
pub const MYANMAR: UnicodeBlock = UnicodeBlock {
    name: "Myanmar", start: 0x1000, end: 0x109F
};
pub const GEORGIAN: UnicodeBlock = UnicodeBlock {
    name: "Georgian", start: 0x10A0, end: 0x10FF
};
pub const HANGUL_JAMO: UnicodeBlock =
    UnicodeBlock {
        name: "Hangul Jamo", start: 0x1100, end: 0x11FF
    };
pub const ETHIOPIC: UnicodeBlock = UnicodeBlock {
    name: "Ethiopic", start: 0x1200, end: 0x137F
};
pub const ETHIOPIC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Ethiopic Supplement", start: 0x1380, end: 0x139F
    };
pub const CHEROKEE: UnicodeBlock = UnicodeBlock {
    name: "Cherokee", start: 0x13A0, end: 0x13FF
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UnicodeBlock =
    UnicodeBlock {
        name: "Unified Canadian Aboriginal Syllabics", start: 0x1400, end: 0x167F
    };
pub const OGHAM: UnicodeBlock = UnicodeBlock {
    name: "Ogham", start: 0x1680, end: 0x169F
};
pub const RUNIC: UnicodeBlock = UnicodeBlock {
    name: "Runic", start: 0x16A0, end: 0x16FF
};
pub const TAGALOG: UnicodeBlock = UnicodeBlock {
    name: "Tagalog", start: 0x1700, end: 0x171F
};
pub const HANUNOO: UnicodeBlock = UnicodeBlock {
    name: "Hanunoo", start: 0x1720, end: 0x173F
};
pub const BUHID: UnicodeBlock = UnicodeBlock {
    name: "Buhid", start: 0x1740, end: 0x175F
};
pub const TAGBANWA: UnicodeBlock = UnicodeBlock {
    name: "Tagbanwa", start: 0x1760, end: 0x177F
};
pub const KHMER: UnicodeBlock = UnicodeBlock {
    name: "Khmer", start: 0x1780, end: 0x17FF
};
pub const MONGOLIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Mongolian", start: 0x1800, end: 0x18AF
    };
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UnicodeBlock = UnicodeBlock {
    name:  "Unified Canadian Aboriginal Syllabics Extended",
    start: 0x18B0,
    end:   0x18FF,
};
pub const LIMBU: UnicodeBlock = UnicodeBlock {
    name: "Limbu", start: 0x1900, end: 0x194F
};
pub const TAI_LE: UnicodeBlock = UnicodeBlock {
    name: "Tai Le", start: 0x1950, end: 0x197F
};
pub const NEW_TAI_LUE: UnicodeBlock =
    UnicodeBlock {
        name: "New Tai Lue", start: 0x1980, end: 0x19DF
    };
pub const KHMER_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Khmer Symbols", start: 0x19E0, end: 0x19FF
    };
pub const BUGINESE: UnicodeBlock = UnicodeBlock {
    name: "Buginese", start: 0x1A00, end: 0x1A1F
};
pub const TAI_THAM: UnicodeBlock = UnicodeBlock {
    name: "Tai Tham", start: 0x1A20, end: 0x1AAF
};
pub const COMBINING_DIACRITICAL_MARKS_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Combining Diacritical Marks Extended", start: 0x1AB0, end: 0x1AFF
    };
pub const BALINESE: UnicodeBlock = UnicodeBlock {
    name: "Balinese", start: 0x1B00, end: 0x1B7F
};
pub const SUNDANESE: UnicodeBlock =
    UnicodeBlock {
        name: "Sundanese", start: 0x1B80, end: 0x1BBF
    };
pub const BATAK: UnicodeBlock = UnicodeBlock {
    name: "Batak", start: 0x1BC0, end: 0x1BFF
};
pub const LEPCHA: UnicodeBlock = UnicodeBlock {
    name: "Lepcha", start: 0x1C00, end: 0x1C4F
};
pub const OL_CHIKI: UnicodeBlock = UnicodeBlock {
    name: "Ol Chiki", start: 0x1C50, end: 0x1C7F
};
pub const CYRILLIC_EXTENDED_C: UnicodeBlock =
    UnicodeBlock {
        name: "Cyrillic Extended-C", start: 0x1C80, end: 0x1C8F
    };
pub const GEORGIAN_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Georgian Extended", start: 0x1C90, end: 0x1CBF
    };
pub const SUNDANESE_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Sundanese Supplement", start: 0x1CC0, end: 0x1CCF
    };
pub const VEDIC_EXTENSIONS: UnicodeBlock =
    UnicodeBlock {
        name: "Vedic Extensions", start: 0x1CD0, end: 0x1CFF
    };
pub const PHONETIC_EXTENSIONS: UnicodeBlock =
    UnicodeBlock {
        name: "Phonetic Extensions", start: 0x1D00, end: 0x1D7F
    };
pub const PHONETIC_EXTENSIONS_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Phonetic Extensions Supplement", start: 0x1D80, end: 0x1DBF
    };
pub const COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Combining Diacritical Marks Supplement", start: 0x1DC0, end: 0x1DFF
    };
pub const LATIN_EXTENDED_ADDITIONAL: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended Additional", start: 0x1E00, end: 0x1EFF
    };
pub const GREEK_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Greek Extended", start: 0x1F00, end: 0x1FFF
    };
pub const GENERAL_PUNCTUATION: UnicodeBlock =
    UnicodeBlock {
        name: "General Punctuation", start: 0x2000, end: 0x206F
    };
pub const SUPERSCRIPTS_AND_SUBSCRIPTS: UnicodeBlock =
    UnicodeBlock {
        name: "Superscripts and Subscripts", start: 0x2070, end: 0x209F
    };
pub const CURRENCY_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Currency Symbols", start: 0x20A0, end: 0x20CF
    };
pub const COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Combining Diacritical Marks for Symbols", start: 0x20D0, end: 0x20FF
    };
pub const LETTERLIKE_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Letterlike Symbols", start: 0x2100, end: 0x214F
    };
pub const NUMBER_FORMS: UnicodeBlock =
    UnicodeBlock {
        name: "Number Forms", start: 0x2150, end: 0x218F
    };
pub const ARROWS: UnicodeBlock = UnicodeBlock {
    name: "Arrows", start: 0x2190, end: 0x21FF
};
pub const MATHEMATICAL_OPERATORS: UnicodeBlock =
    UnicodeBlock {
        name: "Mathematical Operators", start: 0x2200, end: 0x22FF
    };
pub const MISCELLANEOUS_TECHNICAL: UnicodeBlock =
    UnicodeBlock {
        name: "Miscellaneous Technical", start: 0x2300, end: 0x23FF
    };
pub const CONTROL_PICTURES: UnicodeBlock =
    UnicodeBlock {
        name: "Control Pictures", start: 0x2400, end: 0x243F
    };
pub const OPTICAL_CHARACTER_RECOGNITION: UnicodeBlock =
    UnicodeBlock {
        name: "Optical Character Recognition", start: 0x2440, end: 0x245F
    };
pub const ENCLOSED_ALPHANUMERICS: UnicodeBlock =
    UnicodeBlock {
        name: "Enclosed Alphanumerics", start: 0x2460, end: 0x24FF
    };
pub const BOX_DRAWING: UnicodeBlock =
    UnicodeBlock {
        name: "Box Drawing", start: 0x2500, end: 0x257F
    };
pub const BLOCK_ELEMENTS: UnicodeBlock =
    UnicodeBlock {
        name: "Block Elements", start: 0x2580, end: 0x259F
    };
pub const GEOMETRIC_SHAPES: UnicodeBlock =
    UnicodeBlock {
        name: "Geometric Shapes", start: 0x25A0, end: 0x25FF
    };
pub const MISCELLANEOUS_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Miscellaneous Symbols", start: 0x2600, end: 0x26FF
    };
pub const DINGBATS: UnicodeBlock = UnicodeBlock {
    name: "Dingbats", start: 0x2700, end: 0x27BF
};
pub const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UnicodeBlock =
    UnicodeBlock {
        name: "Miscellaneous Mathematical Symbols-A", start: 0x27C0, end: 0x27EF
    };
pub const SUPPLEMENTAL_ARROWS_A: UnicodeBlock =
    UnicodeBlock {
        name: "Supplemental Arrows-A", start: 0x27F0, end: 0x27FF
    };
pub const BRAILLE_PATTERNS: UnicodeBlock =
    UnicodeBlock {
        name: "Braille Patterns", start: 0x2800, end: 0x28FF
    };
pub const SUPPLEMENTAL_ARROWS_B: UnicodeBlock =
    UnicodeBlock {
        name: "Supplemental Arrows-B", start: 0x2900, end: 0x297F
    };
pub const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UnicodeBlock =
    UnicodeBlock {
        name: "Miscellaneous Mathematical Symbols-B", start: 0x2980, end: 0x29FF
    };
pub const SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UnicodeBlock =
    UnicodeBlock {
        name: "Supplemental Mathematical Operators", start: 0x2A00, end: 0x2AFF
    };
pub const MISCELLANEOUS_SYMBOLS_AND_ARROWS: UnicodeBlock =
    UnicodeBlock {
        name: "Miscellaneous Symbols and Arrows", start: 0x2B00, end: 0x2BFF
    };
pub const GLAGOLITIC: UnicodeBlock =
    UnicodeBlock {
        name: "Glagolitic", start: 0x2C00, end: 0x2C5F
    };
pub const LATIN_EXTENDED_C: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-C", start: 0x2C60, end: 0x2C7F
    };
pub const COPTIC: UnicodeBlock = UnicodeBlock {
    name: "Coptic", start: 0x2C80, end: 0x2CFF
};
pub const GEORGIAN_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Georgian Supplement", start: 0x2D00, end: 0x2D2F
    };
pub const TIFINAGH: UnicodeBlock = UnicodeBlock {
    name: "Tifinagh", start: 0x2D30, end: 0x2D7F
};
pub const ETHIOPIC_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Ethiopic Extended", start: 0x2D80, end: 0x2DDF
    };
pub const CYRILLIC_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Cyrillic Extended-A", start: 0x2DE0, end: 0x2DFF
    };
pub const SUPPLEMENTAL_PUNCTUATION: UnicodeBlock =
    UnicodeBlock {
        name: "Supplemental Punctuation", start: 0x2E00, end: 0x2E7F
    };
pub const CJK_RADICALS_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Radicals Supplement", start: 0x2E80, end: 0x2EFF
    };
pub const KANGXI_RADICALS: UnicodeBlock =
    UnicodeBlock {
        name: "Kangxi Radicals", start: 0x2F00, end: 0x2FDF
    };
pub const IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UnicodeBlock =
    UnicodeBlock {
        name: "Ideographic Description Characters", start: 0x2FF0, end: 0x2FFF
    };
pub const CJK_SYMBOLS_AND_PUNCTUATION: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Symbols and Punctuation", start: 0x3000, end: 0x303F
    };
pub const HIRAGANA: UnicodeBlock = UnicodeBlock {
    name: "Hiragana", start: 0x3040, end: 0x309F
};
pub const KATAKANA: UnicodeBlock = UnicodeBlock {
    name: "Katakana", start: 0x30A0, end: 0x30FF
};
pub const BOPOMOFO: UnicodeBlock = UnicodeBlock {
    name: "Bopomofo", start: 0x3100, end: 0x312F
};
pub const HANGUL_COMPATIBILITY_JAMO: UnicodeBlock =
    UnicodeBlock {
        name: "Hangul Compatibility Jamo", start: 0x3130, end: 0x318F
    };
pub const KANBUN: UnicodeBlock = UnicodeBlock {
    name: "Kanbun", start: 0x3190, end: 0x319F
};
pub const BOPOMOFO_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Bopomofo Extended", start: 0x31A0, end: 0x31BF
    };
pub const CJK_STROKES: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Strokes", start: 0x31C0, end: 0x31EF
    };
pub const KATAKANA_PHONETIC_EXTENSIONS: UnicodeBlock =
    UnicodeBlock {
        name: "Katakana Phonetic Extensions", start: 0x31F0, end: 0x31FF
    };
pub const ENCLOSED_CJK_LETTERS_AND_MONTHS: UnicodeBlock =
    UnicodeBlock {
        name: "Enclosed CJK Letters and Months", start: 0x3200, end: 0x32FF
    };
pub const CJK_COMPATIBILITY: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Compatibility", start: 0x3300, end: 0x33FF
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension A", start: 0x3400, end: 0x4DBF
    };
pub const YIJING_HEXAGRAM_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Yijing Hexagram Symbols", start: 0x4DC0, end: 0x4DFF
    };
pub const CJK_UNIFIED_IDEOGRAPHS: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs", start: 0x4E00, end: 0x9FFF
    };
pub const YI_SYLLABLES: UnicodeBlock =
    UnicodeBlock {
        name: "Yi Syllables", start: 0xA000, end: 0xA48F
    };
pub const YI_RADICALS: UnicodeBlock =
    UnicodeBlock {
        name: "Yi Radicals", start: 0xA490, end: 0xA4CF
    };
pub const LISU: UnicodeBlock = UnicodeBlock {
    name: "Lisu", start: 0xA4D0, end: 0xA4FF
};
pub const VAI: UnicodeBlock = UnicodeBlock {
    name: "Vai", start: 0xA500, end: 0xA63F
};
pub const CYRILLIC_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Cyrillic Extended-B", start: 0xA640, end: 0xA69F
    };
pub const BAMUM: UnicodeBlock = UnicodeBlock {
    name: "Bamum", start: 0xA6A0, end: 0xA6FF
};
pub const MODIFIER_TONE_LETTERS: UnicodeBlock =
    UnicodeBlock {
        name: "Modifier Tone Letters", start: 0xA700, end: 0xA71F
    };
pub const LATIN_EXTENDED_D: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-D", start: 0xA720, end: 0xA7FF
    };
pub const SYLOTI_NAGRI: UnicodeBlock =
    UnicodeBlock {
        name: "Syloti Nagri", start: 0xA800, end: 0xA82F
    };
pub const COMMON_INDIC_NUMBER_FORMS: UnicodeBlock =
    UnicodeBlock {
        name: "Common Indic Number Forms", start: 0xA830, end: 0xA83F
    };
pub const PHAGS_PA: UnicodeBlock = UnicodeBlock {
    name: "Phags-pa", start: 0xA840, end: 0xA87F
};
pub const SAURASHTRA: UnicodeBlock =
    UnicodeBlock {
        name: "Saurashtra", start: 0xA880, end: 0xA8DF
    };
pub const DEVANAGARI_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Devanagari Extended", start: 0xA8E0, end: 0xA8FF
    };
pub const KAYAH_LI: UnicodeBlock = UnicodeBlock {
    name: "Kayah Li", start: 0xA900, end: 0xA92F
};
pub const REJANG: UnicodeBlock = UnicodeBlock {
    name: "Rejang", start: 0xA930, end: 0xA95F
};
pub const HANGUL_JAMO_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Hangul Jamo Extended-A", start: 0xA960, end: 0xA97F
    };
pub const JAVANESE: UnicodeBlock = UnicodeBlock {
    name: "Javanese", start: 0xA980, end: 0xA9DF
};
pub const MYANMAR_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Myanmar Extended-B", start: 0xA9E0, end: 0xA9FF
    };
pub const CHAM: UnicodeBlock = UnicodeBlock {
    name: "Cham", start: 0xAA00, end: 0xAA5F
};
pub const MYANMAR_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Myanmar Extended-A", start: 0xAA60, end: 0xAA7F
    };
pub const TAI_VIET: UnicodeBlock = UnicodeBlock {
    name: "Tai Viet", start: 0xAA80, end: 0xAADF
};
pub const MEETEI_MAYEK_EXTENSIONS: UnicodeBlock =
    UnicodeBlock {
        name: "Meetei Mayek Extensions", start: 0xAAE0, end: 0xAAFF
    };
pub const ETHIOPIC_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Ethiopic Extended-A", start: 0xAB00, end: 0xAB2F
    };
pub const LATIN_EXTENDED_E: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-E", start: 0xAB30, end: 0xAB6F
    };
pub const CHEROKEE_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Cherokee Supplement", start: 0xAB70, end: 0xABBF
    };
pub const MEETEI_MAYEK: UnicodeBlock =
    UnicodeBlock {
        name: "Meetei Mayek", start: 0xABC0, end: 0xABFF
    };
pub const HANGUL_SYLLABLES: UnicodeBlock =
    UnicodeBlock {
        name: "Hangul Syllables", start: 0xAC00, end: 0xD7AF
    };
pub const HANGUL_JAMO_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Hangul Jamo Extended-B", start: 0xD7B0, end: 0xD7FF
    };
pub const HIGH_SURROGATES: UnicodeBlock =
    UnicodeBlock {
        name: "High Surrogates", start: 0xD800, end: 0xDB7F
    };
pub const HIGH_PRIVATE_USE_SURROGATES: UnicodeBlock =
    UnicodeBlock {
        name: "High Private Use Surrogates", start: 0xDB80, end: 0xDBFF
    };
pub const LOW_SURROGATES: UnicodeBlock =
    UnicodeBlock {
        name: "Low Surrogates", start: 0xDC00, end: 0xDFFF
    };
pub const PRIVATE_USE_AREA: UnicodeBlock =
    UnicodeBlock {
        name: "Private Use Area", start: 0xE000, end: 0xF8FF
    };
pub const CJK_COMPATIBILITY_IDEOGRAPHS: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Compatibility Ideographs", start: 0xF900, end: 0xFAFF
    };
pub const ALPHABETIC_PRESENTATION_FORMS: UnicodeBlock =
    UnicodeBlock {
        name: "Alphabetic Presentation Forms", start: 0xFB00, end: 0xFB4F
    };
pub const ARABIC_PRESENTATION_FORMS_A: UnicodeBlock =
    UnicodeBlock {
        name: "Arabic Presentation Forms-A", start: 0xFB50, end: 0xFDFF
    };
pub const VARIATION_SELECTORS: UnicodeBlock =
    UnicodeBlock {
        name: "Variation Selectors", start: 0xFE00, end: 0xFE0F
    };
pub const VERTICAL_FORMS: UnicodeBlock =
    UnicodeBlock {
        name: "Vertical Forms", start: 0xFE10, end: 0xFE1F
    };
pub const COMBINING_HALF_MARKS: UnicodeBlock =
    UnicodeBlock {
        name: "Combining Half Marks", start: 0xFE20, end: 0xFE2F
    };
pub const CJK_COMPATIBILITY_FORMS: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Compatibility Forms", start: 0xFE30, end: 0xFE4F
    };
pub const SMALL_FORM_VARIANTS: UnicodeBlock =
    UnicodeBlock {
        name: "Small Form Variants", start: 0xFE50, end: 0xFE6F
    };
pub const ARABIC_PRESENTATION_FORMS_B: UnicodeBlock =
    UnicodeBlock {
        name: "Arabic Presentation Forms-B", start: 0xFE70, end: 0xFEFF
    };
pub const HALFWIDTH_AND_FULLWIDTH_FORMS: UnicodeBlock =
    UnicodeBlock {
        name: "Halfwidth and Fullwidth Forms", start: 0xFF00, end: 0xFFEF
    };
pub const SPECIALS: UnicodeBlock = UnicodeBlock {
    name: "Specials", start: 0xFFF0, end: 0xFFFF
};
pub const LINEAR_B_SYLLABARY: UnicodeBlock =
    UnicodeBlock {
        name: "Linear B Syllabary", start: 0x10000, end: 0x1007F
    };
pub const LINEAR_B_IDEOGRAMS: UnicodeBlock =
    UnicodeBlock {
        name: "Linear B Ideograms", start: 0x10080, end: 0x100FF
    };
pub const AEGEAN_NUMBERS: UnicodeBlock =
    UnicodeBlock {
        name: "Aegean Numbers", start: 0x10100, end: 0x1013F
    };
pub const ANCIENT_GREEK_NUMBERS: UnicodeBlock =
    UnicodeBlock {
        name: "Ancient Greek Numbers", start: 0x10140, end: 0x1018F
    };
pub const ANCIENT_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Ancient Symbols", start: 0x10190, end: 0x101CF
    };
pub const PHAISTOS_DISC: UnicodeBlock =
    UnicodeBlock {
        name: "Phaistos Disc", start: 0x101D0, end: 0x101FF
    };
pub const LYCIAN: UnicodeBlock = UnicodeBlock {
    name: "Lycian", start: 0x10280, end: 0x1029F
};
pub const CARIAN: UnicodeBlock = UnicodeBlock {
    name: "Carian", start: 0x102A0, end: 0x102DF
};
pub const COPTIC_EPACT_NUMBERS: UnicodeBlock =
    UnicodeBlock {
        name: "Coptic Epact Numbers", start: 0x102E0, end: 0x102FF
    };
pub const OLD_ITALIC: UnicodeBlock =
    UnicodeBlock {
        name: "Old Italic", start: 0x10300, end: 0x1032F
    };
pub const GOTHIC: UnicodeBlock = UnicodeBlock {
    name: "Gothic", start: 0x10330, end: 0x1034F
};
pub const OLD_PERMIC: UnicodeBlock =
    UnicodeBlock {
        name: "Old Permic", start: 0x10350, end: 0x1037F
    };
pub const UGARITIC: UnicodeBlock =
    UnicodeBlock {
        name: "Ugaritic", start: 0x10380, end: 0x1039F
    };
pub const OLD_PERSIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Old Persian", start: 0x103A0, end: 0x103DF
    };
pub const DESERET: UnicodeBlock = UnicodeBlock {
    name: "Deseret", start: 0x10400, end: 0x1044F
};
pub const SHAVIAN: UnicodeBlock = UnicodeBlock {
    name: "Shavian", start: 0x10450, end: 0x1047F
};
pub const OSMANYA: UnicodeBlock = UnicodeBlock {
    name: "Osmanya", start: 0x10480, end: 0x104AF
};
pub const OSAGE: UnicodeBlock = UnicodeBlock {
    name: "Osage", start: 0x104B0, end: 0x104FF
};
pub const ELBASAN: UnicodeBlock = UnicodeBlock {
    name: "Elbasan", start: 0x10500, end: 0x1052F
};
pub const CAUCASIAN_ALBANIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Caucasian Albanian", start: 0x10530, end: 0x1056F
    };
pub const VITHKUQI: UnicodeBlock =
    UnicodeBlock {
        name: "Vithkuqi", start: 0x10570, end: 0x105BF
    };
pub const TODHRI: UnicodeBlock = UnicodeBlock {
    name: "Todhri", start: 0x105C0, end: 0x105FF
};
pub const LINEAR_A: UnicodeBlock =
    UnicodeBlock {
        name: "Linear A", start: 0x10600, end: 0x1077F
    };
pub const LATIN_EXTENDED_F: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-F", start: 0x10780, end: 0x107BF
    };
pub const CYPRIOT_SYLLABARY: UnicodeBlock =
    UnicodeBlock {
        name: "Cypriot Syllabary", start: 0x10800, end: 0x1083F
    };
pub const IMPERIAL_ARAMAIC: UnicodeBlock =
    UnicodeBlock {
        name: "Imperial Aramaic", start: 0x10840, end: 0x1085F
    };
pub const PALMYRENE: UnicodeBlock =
    UnicodeBlock {
        name: "Palmyrene", start: 0x10860, end: 0x1087F
    };
pub const NABATAEAN: UnicodeBlock =
    UnicodeBlock {
        name: "Nabataean", start: 0x10880, end: 0x108AF
    };
pub const HATRAN: UnicodeBlock = UnicodeBlock {
    name: "Hatran", start: 0x108E0, end: 0x108FF
};
pub const PHOENICIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Phoenician", start: 0x10900, end: 0x1091F
    };
pub const LYDIAN: UnicodeBlock = UnicodeBlock {
    name: "Lydian", start: 0x10920, end: 0x1093F
};
pub const MEROITIC_HIEROGLYPHS: UnicodeBlock =
    UnicodeBlock {
        name: "Meroitic Hieroglyphs", start: 0x10980, end: 0x1099F
    };
pub const MEROITIC_CURSIVE: UnicodeBlock =
    UnicodeBlock {
        name: "Meroitic Cursive", start: 0x109A0, end: 0x109FF
    };
pub const KHAROSHTHI: UnicodeBlock =
    UnicodeBlock {
        name: "Kharoshthi", start: 0x10A00, end: 0x10A5F
    };
pub const OLD_SOUTH_ARABIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Old South Arabian", start: 0x10A60, end: 0x10A7F
    };
pub const OLD_NORTH_ARABIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Old North Arabian", start: 0x10A80, end: 0x10A9F
    };
pub const MANICHAEAN: UnicodeBlock =
    UnicodeBlock {
        name: "Manichaean", start: 0x10AC0, end: 0x10AFF
    };
pub const AVESTAN: UnicodeBlock = UnicodeBlock {
    name: "Avestan", start: 0x10B00, end: 0x10B3F
};
pub const INSCRIPTIONAL_PARTHIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Inscriptional Parthian", start: 0x10B40, end: 0x10B5F
    };
pub const INSCRIPTIONAL_PAHLAVI: UnicodeBlock =
    UnicodeBlock {
        name: "Inscriptional Pahlavi", start: 0x10B60, end: 0x10B7F
    };
pub const PSALTER_PAHLAVI: UnicodeBlock =
    UnicodeBlock {
        name: "Psalter Pahlavi", start: 0x10B80, end: 0x10BAF
    };
pub const OLD_TURKIC: UnicodeBlock =
    UnicodeBlock {
        name: "Old Turkic", start: 0x10C00, end: 0x10C4F
    };
pub const OLD_HUNGARIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Old Hungarian", start: 0x10C80, end: 0x10CFF
    };
pub const HANIFI_ROHINGYA: UnicodeBlock =
    UnicodeBlock {
        name: "Hanifi Rohingya", start: 0x10D00, end: 0x10D3F
    };
pub const GARAY: UnicodeBlock = UnicodeBlock {
    name: "Garay", start: 0x10D40, end: 0x10D8F
};
pub const RUMI_NUMERAL_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Rumi Numeral Symbols", start: 0x10E60, end: 0x10E7F
    };
pub const YEZIDI: UnicodeBlock = UnicodeBlock {
    name: "Yezidi", start: 0x10E80, end: 0x10EBF
};
pub const ARABIC_EXTENDED_C: UnicodeBlock =
    UnicodeBlock {
        name: "Arabic Extended-C", start: 0x10EC0, end: 0x10EFF
    };
pub const OLD_SOGDIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Old Sogdian", start: 0x10F00, end: 0x10F2F
    };
pub const SOGDIAN: UnicodeBlock = UnicodeBlock {
    name: "Sogdian", start: 0x10F30, end: 0x10F6F
};
pub const OLD_UYGHUR: UnicodeBlock =
    UnicodeBlock {
        name: "Old Uyghur", start: 0x10F70, end: 0x10FAF
    };
pub const CHORASMIAN: UnicodeBlock =
    UnicodeBlock {
        name: "Chorasmian", start: 0x10FB0, end: 0x10FDF
    };
pub const ELYMAIC: UnicodeBlock = UnicodeBlock {
    name: "Elymaic", start: 0x10FE0, end: 0x10FFF
};
pub const BRAHMI: UnicodeBlock = UnicodeBlock {
    name: "Brahmi", start: 0x11000, end: 0x1107F
};
pub const KAITHI: UnicodeBlock = UnicodeBlock {
    name: "Kaithi", start: 0x11080, end: 0x110CF
};
pub const SORA_SOMPENG: UnicodeBlock =
    UnicodeBlock {
        name: "Sora Sompeng", start: 0x110D0, end: 0x110FF
    };
pub const CHAKMA: UnicodeBlock = UnicodeBlock {
    name: "Chakma", start: 0x11100, end: 0x1114F
};
pub const MAHAJANI: UnicodeBlock =
    UnicodeBlock {
        name: "Mahajani", start: 0x11150, end: 0x1117F
    };
pub const SHARADA: UnicodeBlock = UnicodeBlock {
    name: "Sharada", start: 0x11180, end: 0x111DF
};
pub const SINHALA_ARCHAIC_NUMBERS: UnicodeBlock =
    UnicodeBlock {
        name: "Sinhala Archaic Numbers", start: 0x111E0, end: 0x111FF
    };
pub const KHOJKI: UnicodeBlock = UnicodeBlock {
    name: "Khojki", start: 0x11200, end: 0x1124F
};
pub const MULTANI: UnicodeBlock = UnicodeBlock {
    name: "Multani", start: 0x11280, end: 0x112AF
};
pub const KHUDAWADI: UnicodeBlock =
    UnicodeBlock {
        name: "Khudawadi", start: 0x112B0, end: 0x112FF
    };
pub const GRANTHA: UnicodeBlock = UnicodeBlock {
    name: "Grantha", start: 0x11300, end: 0x1137F
};
pub const TULU_TIGALARI: UnicodeBlock =
    UnicodeBlock {
        name: "Tulu-Tigalari", start: 0x11380, end: 0x113FF
    };
pub const NEWA: UnicodeBlock = UnicodeBlock {
    name: "Newa", start: 0x11400, end: 0x1147F
};
pub const TIRHUTA: UnicodeBlock = UnicodeBlock {
    name: "Tirhuta", start: 0x11480, end: 0x114DF
};
pub const SIDDHAM: UnicodeBlock = UnicodeBlock {
    name: "Siddham", start: 0x11580, end: 0x115FF
};
pub const MODI: UnicodeBlock = UnicodeBlock {
    name: "Modi", start: 0x11600, end: 0x1165F
};
pub const MONGOLIAN_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Mongolian Supplement", start: 0x11660, end: 0x1167F
    };
pub const TAKRI: UnicodeBlock = UnicodeBlock {
    name: "Takri", start: 0x11680, end: 0x116CF
};
pub const MYANMAR_EXTENDED_C: UnicodeBlock =
    UnicodeBlock {
        name: "Myanmar Extended-C", start: 0x116D0, end: 0x116FF
    };
pub const AHOM: UnicodeBlock = UnicodeBlock {
    name: "Ahom", start: 0x11700, end: 0x1174F
};
pub const DOGRA: UnicodeBlock = UnicodeBlock {
    name: "Dogra", start: 0x11800, end: 0x1184F
};
pub const WARANG_CITI: UnicodeBlock =
    UnicodeBlock {
        name: "Warang Citi", start: 0x118A0, end: 0x118FF
    };
pub const DIVES_AKURU: UnicodeBlock =
    UnicodeBlock {
        name: "Dives Akuru", start: 0x11900, end: 0x1195F
    };
pub const NANDINAGARI: UnicodeBlock =
    UnicodeBlock {
        name: "Nandinagari", start: 0x119A0, end: 0x119FF
    };
pub const ZANABAZAR_SQUARE: UnicodeBlock =
    UnicodeBlock {
        name: "Zanabazar Square", start: 0x11A00, end: 0x11A4F
    };
pub const SOYOMBO: UnicodeBlock = UnicodeBlock {
    name: "Soyombo", start: 0x11A50, end: 0x11AAF
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name:  "Unified Canadian Aboriginal Syllabics Extended-A",
    start: 0x11AB0,
    end:   0x11ABF,
};
pub const PAU_CIN_HAU: UnicodeBlock =
    UnicodeBlock {
        name: "Pau Cin Hau", start: 0x11AC0, end: 0x11AFF
    };
pub const DEVANAGARI_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Devanagari Extended-A", start: 0x11B00, end: 0x11B5F
    };
pub const SUNUWAR: UnicodeBlock = UnicodeBlock {
    name: "Sunuwar", start: 0x11BC0, end: 0x11BFF
};
pub const BHAIKSUKI: UnicodeBlock =
    UnicodeBlock {
        name: "Bhaiksuki", start: 0x11C00, end: 0x11C6F
    };
pub const MARCHEN: UnicodeBlock = UnicodeBlock {
    name: "Marchen", start: 0x11C70, end: 0x11CBF
};
pub const MASARAM_GONDI: UnicodeBlock =
    UnicodeBlock {
        name: "Masaram Gondi", start: 0x11D00, end: 0x11D5F
    };
pub const GUNJALA_GONDI: UnicodeBlock =
    UnicodeBlock {
        name: "Gunjala Gondi", start: 0x11D60, end: 0x11DAF
    };
pub const MAKASAR: UnicodeBlock = UnicodeBlock {
    name: "Makasar", start: 0x11EE0, end: 0x11EFF
};
pub const KAWI: UnicodeBlock = UnicodeBlock {
    name: "Kawi", start: 0x11F00, end: 0x11F5F
};
pub const LISU_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Lisu Supplement", start: 0x11FB0, end: 0x11FBF
    };
pub const TAMIL_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Tamil Supplement", start: 0x11FC0, end: 0x11FFF
    };
pub const CUNEIFORM: UnicodeBlock =
    UnicodeBlock {
        name: "Cuneiform", start: 0x12000, end: 0x123FF
    };
pub const CUNEIFORM_NUMBERS_AND_PUNCTUATION: UnicodeBlock =
    UnicodeBlock {
        name: "Cuneiform Numbers and Punctuation", start: 0x12400, end: 0x1247F
    };
pub const EARLY_DYNASTIC_CUNEIFORM: UnicodeBlock =
    UnicodeBlock {
        name: "Early Dynastic Cuneiform", start: 0x12480, end: 0x1254F
    };
pub const CYPRO_MINOAN: UnicodeBlock =
    UnicodeBlock {
        name: "Cypro-Minoan", start: 0x12F90, end: 0x12FFF
    };
pub const EGYPTIAN_HIEROGLYPHS: UnicodeBlock =
    UnicodeBlock {
        name: "Egyptian Hieroglyphs", start: 0x13000, end: 0x1342F
    };
pub const EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UnicodeBlock =
    UnicodeBlock {
        name: "Egyptian Hieroglyph Format Controls", start: 0x13430, end: 0x1345F
    };
pub const EGYPTIAN_HIEROGLYPHS_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Egyptian Hieroglyphs Extended-A", start: 0x13460, end: 0x143FF
    };
pub const ANATOLIAN_HIEROGLYPHS: UnicodeBlock =
    UnicodeBlock {
        name: "Anatolian Hieroglyphs", start: 0x14400, end: 0x1467F
    };
pub const GURUNG_KHEMA: UnicodeBlock =
    UnicodeBlock {
        name: "Gurung Khema", start: 0x16100, end: 0x1613F
    };
pub const BAMUM_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Bamum Supplement", start: 0x16800, end: 0x16A3F
    };
pub const MRO: UnicodeBlock = UnicodeBlock {
    name: "Mro", start: 0x16A40, end: 0x16A6F
};
pub const TANGSA: UnicodeBlock = UnicodeBlock {
    name: "Tangsa", start: 0x16A70, end: 0x16ACF
};
pub const BASSA_VAH: UnicodeBlock =
    UnicodeBlock {
        name: "Bassa Vah", start: 0x16AD0, end: 0x16AFF
    };
pub const PAHAWH_HMONG: UnicodeBlock =
    UnicodeBlock {
        name: "Pahawh Hmong", start: 0x16B00, end: 0x16B8F
    };
pub const KIRAT_RAI: UnicodeBlock =
    UnicodeBlock {
        name: "Kirat Rai", start: 0x16D40, end: 0x16D7F
    };
pub const MEDEFAIDRIN: UnicodeBlock =
    UnicodeBlock {
        name: "Medefaidrin", start: 0x16E40, end: 0x16E9F
    };
pub const MIAO: UnicodeBlock = UnicodeBlock {
    name: "Miao", start: 0x16F00, end: 0x16F9F
};
pub const IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UnicodeBlock =
    UnicodeBlock {
        name: "Ideographic Symbols and Punctuation", start: 0x16FE0, end: 0x16FFF
    };
pub const TANGUT: UnicodeBlock = UnicodeBlock {
    name: "Tangut", start: 0x17000, end: 0x187FF
};
pub const TANGUT_COMPONENTS: UnicodeBlock =
    UnicodeBlock {
        name: "Tangut Components", start: 0x18800, end: 0x18AFF
    };
pub const KHITAN_SMALL_SCRIPT: UnicodeBlock =
    UnicodeBlock {
        name: "Khitan Small Script", start: 0x18B00, end: 0x18CFF
    };
pub const TANGUT_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Tangut Supplement", start: 0x18D00, end: 0x18D7F
    };
pub const KANA_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Kana Extended-B", start: 0x1AFF0, end: 0x1AFFF
    };
pub const KANA_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Kana Supplement", start: 0x1B000, end: 0x1B0FF
    };
pub const KANA_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Kana Extended-A", start: 0x1B100, end: 0x1B12F
    };
pub const SMALL_KANA_EXTENSION: UnicodeBlock =
    UnicodeBlock {
        name: "Small Kana Extension", start: 0x1B130, end: 0x1B16F
    };
pub const NUSHU: UnicodeBlock = UnicodeBlock {
    name: "Nushu", start: 0x1B170, end: 0x1B2FF
};
pub const DUPLOYAN: UnicodeBlock =
    UnicodeBlock {
        name: "Duployan", start: 0x1BC00, end: 0x1BC9F
    };
pub const SHORTHAND_FORMAT_CONTROLS: UnicodeBlock =
    UnicodeBlock {
        name: "Shorthand Format Controls", start: 0x1BCA0, end: 0x1BCAF
    };
pub const SYMBOLS_FOR_LEGACY_COMPUTING_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name:  "Symbols for Legacy Computing Supplement",
    start: 0x1CC00,
    end:   0x1CEBF,
};
pub const ZNAMENNY_MUSICAL_NOTATION: UnicodeBlock =
    UnicodeBlock {
        name: "Znamenny Musical Notation", start: 0x1CF00, end: 0x1CFCF
    };
pub const BYZANTINE_MUSICAL_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Byzantine Musical Symbols", start: 0x1D000, end: 0x1D0FF
    };
pub const MUSICAL_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Musical Symbols", start: 0x1D100, end: 0x1D1FF
    };
pub const ANCIENT_GREEK_MUSICAL_NOTATION: UnicodeBlock =
    UnicodeBlock {
        name: "Ancient Greek Musical Notation", start: 0x1D200, end: 0x1D24F
    };
pub const KAKTOVIK_NUMERALS: UnicodeBlock =
    UnicodeBlock {
        name: "Kaktovik Numerals", start: 0x1D2C0, end: 0x1D2DF
    };
pub const MAYAN_NUMERALS: UnicodeBlock =
    UnicodeBlock {
        name: "Mayan Numerals", start: 0x1D2E0, end: 0x1D2FF
    };
pub const TAI_XUAN_JING_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Tai Xuan Jing Symbols", start: 0x1D300, end: 0x1D35F
    };
pub const COUNTING_ROD_NUMERALS: UnicodeBlock =
    UnicodeBlock {
        name: "Counting Rod Numerals", start: 0x1D360, end: 0x1D37F
    };
pub const MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Mathematical Alphanumeric Symbols", start: 0x1D400, end: 0x1D7FF
    };
pub const SUTTON_SIGNWRITING: UnicodeBlock =
    UnicodeBlock {
        name: "Sutton SignWriting", start: 0x1D800, end: 0x1DAAF
    };
pub const LATIN_EXTENDED_G: UnicodeBlock =
    UnicodeBlock {
        name: "Latin Extended-G", start: 0x1DF00, end: 0x1DFFF
    };
pub const GLAGOLITIC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Glagolitic Supplement", start: 0x1E000, end: 0x1E02F
    };
pub const CYRILLIC_EXTENDED_D: UnicodeBlock =
    UnicodeBlock {
        name: "Cyrillic Extended-D", start: 0x1E030, end: 0x1E08F
    };
pub const NYIAKENG_PUACHUE_HMONG: UnicodeBlock =
    UnicodeBlock {
        name: "Nyiakeng Puachue Hmong", start: 0x1E100, end: 0x1E14F
    };
pub const TOTO: UnicodeBlock = UnicodeBlock {
    name: "Toto", start: 0x1E290, end: 0x1E2BF
};
pub const WANCHO: UnicodeBlock = UnicodeBlock {
    name: "Wancho", start: 0x1E2C0, end: 0x1E2FF
};
pub const NAG_MUNDARI: UnicodeBlock =
    UnicodeBlock {
        name: "Nag Mundari", start: 0x1E4D0, end: 0x1E4FF
    };
pub const OL_ONAL: UnicodeBlock = UnicodeBlock {
    name: "Ol Onal", start: 0x1E5D0, end: 0x1E5FF
};
pub const ETHIOPIC_EXTENDED_B: UnicodeBlock =
    UnicodeBlock {
        name: "Ethiopic Extended-B", start: 0x1E7E0, end: 0x1E7FF
    };
pub const MENDE_KIKAKUI: UnicodeBlock =
    UnicodeBlock {
        name: "Mende Kikakui", start: 0x1E800, end: 0x1E8DF
    };
pub const ADLAM: UnicodeBlock = UnicodeBlock {
    name: "Adlam", start: 0x1E900, end: 0x1E95F
};
pub const INDIC_SIYAQ_NUMBERS: UnicodeBlock =
    UnicodeBlock {
        name: "Indic Siyaq Numbers", start: 0x1EC70, end: 0x1ECBF
    };
pub const OTTOMAN_SIYAQ_NUMBERS: UnicodeBlock =
    UnicodeBlock {
        name: "Ottoman Siyaq Numbers", start: 0x1ED00, end: 0x1ED4F
    };
pub const ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name:  "Arabic Mathematical Alphabetic Symbols",
    start: 0x1EE00,
    end:   0x1EEFF,
};
pub const MAHJONG_TILES: UnicodeBlock =
    UnicodeBlock {
        name: "Mahjong Tiles", start: 0x1F000, end: 0x1F02F
    };
pub const DOMINO_TILES: UnicodeBlock =
    UnicodeBlock {
        name: "Domino Tiles", start: 0x1F030, end: 0x1F09F
    };
pub const PLAYING_CARDS: UnicodeBlock =
    UnicodeBlock {
        name: "Playing Cards", start: 0x1F0A0, end: 0x1F0FF
    };
pub const ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Enclosed Alphanumeric Supplement", start: 0x1F100, end: 0x1F1FF
    };
pub const ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Enclosed Ideographic Supplement", start: 0x1F200, end: 0x1F2FF
    };
pub const MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UnicodeBlock =
    UnicodeBlock {
        name: "Miscellaneous Symbols and Pictographs", start: 0x1F300, end: 0x1F5FF
    };
pub const EMOTICONS: UnicodeBlock =
    UnicodeBlock {
        name: "Emoticons", start: 0x1F600, end: 0x1F64F
    };
pub const ORNAMENTAL_DINGBATS: UnicodeBlock =
    UnicodeBlock {
        name: "Ornamental Dingbats", start: 0x1F650, end: 0x1F67F
    };
pub const TRANSPORT_AND_MAP_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Transport and Map Symbols", start: 0x1F680, end: 0x1F6FF
    };
pub const ALCHEMICAL_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Alchemical Symbols", start: 0x1F700, end: 0x1F77F
    };
pub const GEOMETRIC_SHAPES_EXTENDED: UnicodeBlock =
    UnicodeBlock {
        name: "Geometric Shapes Extended", start: 0x1F780, end: 0x1F7FF
    };
pub const SUPPLEMENTAL_ARROWS_C: UnicodeBlock =
    UnicodeBlock {
        name: "Supplemental Arrows-C", start: 0x1F800, end: 0x1F8FF
    };
pub const SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UnicodeBlock =
    UnicodeBlock {
        name: "Supplemental Symbols and Pictographs", start: 0x1F900, end: 0x1F9FF
    };
pub const CHESS_SYMBOLS: UnicodeBlock =
    UnicodeBlock {
        name: "Chess Symbols", start: 0x1FA00, end: 0x1FA6F
    };
pub const SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UnicodeBlock =
    UnicodeBlock {
        name: "Symbols and Pictographs Extended-A", start: 0x1FA70, end: 0x1FAFF
    };
pub const SYMBOLS_FOR_LEGACY_COMPUTING: UnicodeBlock =
    UnicodeBlock {
        name: "Symbols for Legacy Computing", start: 0x1FB00, end: 0x1FBFF
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension B", start: 0x20000, end: 0x2A6DF
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension C", start: 0x2A700, end: 0x2B73F
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension D", start: 0x2B740, end: 0x2B81F
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension E", start: 0x2B820, end: 0x2CEAF
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension F", start: 0x2CEB0, end: 0x2EBEF
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_I: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension I", start: 0x2EBF0, end: 0x2EE5F
    };
pub const CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name:  "CJK Compatibility Ideographs Supplement",
    start: 0x2F800,
    end:   0x2FA1F,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension G", start: 0x30000, end: 0x3134F
    };
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H: UnicodeBlock =
    UnicodeBlock {
        name: "CJK Unified Ideographs Extension H", start: 0x31350, end: 0x323AF
    };
pub const TAGS: UnicodeBlock = UnicodeBlock {
    name: "Tags", start: 0xE0000, end: 0xE007F
};
pub const VARIATION_SELECTORS_SUPPLEMENT: UnicodeBlock =
    UnicodeBlock {
        name: "Variation Selectors Supplement", start: 0xE0100, end: 0xE01EF
    };
pub const SUPPLEMENTARY_PRIVATE_USE_AREA_A: UnicodeBlock =
    UnicodeBlock {
        name: "Supplementary Private Use Area-A", start: 0xF0000, end: 0xFFFFF
    };
pub const SUPPLEMENTARY_PRIVATE_USE_AREA_B: UnicodeBlock =
    UnicodeBlock {
        name: "Supplementary Private Use Area-B", start: 0x100000, end: 0x10FFFF
    };

/// Given a character, determine what unicode block contains it.
pub fn find_unicode_block(c: char) -> Option<UnicodeBlock> {
    match c {
        '\u{0000}'..='\u{007F}' => Some(BASIC_LATIN),
        '\u{0080}'..='\u{00FF}' => Some(LATIN_1_SUPPLEMENT),
        '\u{0100}'..='\u{017F}' => Some(LATIN_EXTENDED_A),
        '\u{0180}'..='\u{024F}' => Some(LATIN_EXTENDED_B),
        '\u{0250}'..='\u{02AF}' => Some(IPA_EXTENSIONS),
        '\u{02B0}'..='\u{02FF}' => Some(SPACING_MODIFIER_LETTERS),
        '\u{0300}'..='\u{036F}' => Some(COMBINING_DIACRITICAL_MARKS),
        '\u{0370}'..='\u{03FF}' => Some(GREEK_AND_COPTIC),
        '\u{0400}'..='\u{04FF}' => Some(CYRILLIC),
        '\u{0500}'..='\u{052F}' => Some(CYRILLIC_SUPPLEMENT),
        '\u{0530}'..='\u{058F}' => Some(ARMENIAN),
        '\u{0590}'..='\u{05FF}' => Some(HEBREW),
        '\u{0600}'..='\u{06FF}' => Some(ARABIC),
        '\u{0700}'..='\u{074F}' => Some(SYRIAC),
        '\u{0750}'..='\u{077F}' => Some(ARABIC_SUPPLEMENT),
        '\u{0780}'..='\u{07BF}' => Some(THAANA),
        '\u{07C0}'..='\u{07FF}' => Some(NKO),
        '\u{0800}'..='\u{083F}' => Some(SAMARITAN),
        '\u{0840}'..='\u{085F}' => Some(MANDAIC),
        '\u{0860}'..='\u{086F}' => Some(SYRIAC_SUPPLEMENT),
        '\u{0870}'..='\u{089F}' => Some(ARABIC_EXTENDED_B),
        '\u{08A0}'..='\u{08FF}' => Some(ARABIC_EXTENDED_A),
        '\u{0900}'..='\u{097F}' => Some(DEVANAGARI),
        '\u{0980}'..='\u{09FF}' => Some(BENGALI),
        '\u{0A00}'..='\u{0A7F}' => Some(GURMUKHI),
        '\u{0A80}'..='\u{0AFF}' => Some(GUJARATI),
        '\u{0B00}'..='\u{0B7F}' => Some(ORIYA),
        '\u{0B80}'..='\u{0BFF}' => Some(TAMIL),
        '\u{0C00}'..='\u{0C7F}' => Some(TELUGU),
        '\u{0C80}'..='\u{0CFF}' => Some(KANNADA),
        '\u{0D00}'..='\u{0D7F}' => Some(MALAYALAM),
        '\u{0D80}'..='\u{0DFF}' => Some(SINHALA),
        '\u{0E00}'..='\u{0E7F}' => Some(THAI),
        '\u{0E80}'..='\u{0EFF}' => Some(LAO),
        '\u{0F00}'..='\u{0FFF}' => Some(TIBETAN),
        '\u{1000}'..='\u{109F}' => Some(MYANMAR),
        '\u{10A0}'..='\u{10FF}' => Some(GEORGIAN),
        '\u{1100}'..='\u{11FF}' => Some(HANGUL_JAMO),
        '\u{1200}'..='\u{137F}' => Some(ETHIOPIC),
        '\u{1380}'..='\u{139F}' => Some(ETHIOPIC_SUPPLEMENT),
        '\u{13A0}'..='\u{13FF}' => Some(CHEROKEE),
        '\u{1400}'..='\u{167F}' => Some(UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS),
        '\u{1680}'..='\u{169F}' => Some(OGHAM),
        '\u{16A0}'..='\u{16FF}' => Some(RUNIC),
        '\u{1700}'..='\u{171F}' => Some(TAGALOG),
        '\u{1720}'..='\u{173F}' => Some(HANUNOO),
        '\u{1740}'..='\u{175F}' => Some(BUHID),
        '\u{1760}'..='\u{177F}' => Some(TAGBANWA),
        '\u{1780}'..='\u{17FF}' => Some(KHMER),
        '\u{1800}'..='\u{18AF}' => Some(MONGOLIAN),
        '\u{18B0}'..='\u{18FF}' => Some(UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED),
        '\u{1900}'..='\u{194F}' => Some(LIMBU),
        '\u{1950}'..='\u{197F}' => Some(TAI_LE),
        '\u{1980}'..='\u{19DF}' => Some(NEW_TAI_LUE),
        '\u{19E0}'..='\u{19FF}' => Some(KHMER_SYMBOLS),
        '\u{1A00}'..='\u{1A1F}' => Some(BUGINESE),
        '\u{1A20}'..='\u{1AAF}' => Some(TAI_THAM),
        '\u{1AB0}'..='\u{1AFF}' => Some(COMBINING_DIACRITICAL_MARKS_EXTENDED),
        '\u{1B00}'..='\u{1B7F}' => Some(BALINESE),
        '\u{1B80}'..='\u{1BBF}' => Some(SUNDANESE),
        '\u{1BC0}'..='\u{1BFF}' => Some(BATAK),
        '\u{1C00}'..='\u{1C4F}' => Some(LEPCHA),
        '\u{1C50}'..='\u{1C7F}' => Some(OL_CHIKI),
        '\u{1C80}'..='\u{1C8F}' => Some(CYRILLIC_EXTENDED_C),
        '\u{1C90}'..='\u{1CBF}' => Some(GEORGIAN_EXTENDED),
        '\u{1CC0}'..='\u{1CCF}' => Some(SUNDANESE_SUPPLEMENT),
        '\u{1CD0}'..='\u{1CFF}' => Some(VEDIC_EXTENSIONS),
        '\u{1D00}'..='\u{1D7F}' => Some(PHONETIC_EXTENSIONS),
        '\u{1D80}'..='\u{1DBF}' => Some(PHONETIC_EXTENSIONS_SUPPLEMENT),
        '\u{1DC0}'..='\u{1DFF}' => Some(COMBINING_DIACRITICAL_MARKS_SUPPLEMENT),
        '\u{1E00}'..='\u{1EFF}' => Some(LATIN_EXTENDED_ADDITIONAL),
        '\u{1F00}'..='\u{1FFF}' => Some(GREEK_EXTENDED),
        '\u{2000}'..='\u{206F}' => Some(GENERAL_PUNCTUATION),
        '\u{2070}'..='\u{209F}' => Some(SUPERSCRIPTS_AND_SUBSCRIPTS),
        '\u{20A0}'..='\u{20CF}' => Some(CURRENCY_SYMBOLS),
        '\u{20D0}'..='\u{20FF}' => Some(COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS),
        '\u{2100}'..='\u{214F}' => Some(LETTERLIKE_SYMBOLS),
        '\u{2150}'..='\u{218F}' => Some(NUMBER_FORMS),
        '\u{2190}'..='\u{21FF}' => Some(ARROWS),
        '\u{2200}'..='\u{22FF}' => Some(MATHEMATICAL_OPERATORS),
        '\u{2300}'..='\u{23FF}' => Some(MISCELLANEOUS_TECHNICAL),
        '\u{2400}'..='\u{243F}' => Some(CONTROL_PICTURES),
        '\u{2440}'..='\u{245F}' => Some(OPTICAL_CHARACTER_RECOGNITION),
        '\u{2460}'..='\u{24FF}' => Some(ENCLOSED_ALPHANUMERICS),
        '\u{2500}'..='\u{257F}' => Some(BOX_DRAWING),
        '\u{2580}'..='\u{259F}' => Some(BLOCK_ELEMENTS),
        '\u{25A0}'..='\u{25FF}' => Some(GEOMETRIC_SHAPES),
        '\u{2600}'..='\u{26FF}' => Some(MISCELLANEOUS_SYMBOLS),
        '\u{2700}'..='\u{27BF}' => Some(DINGBATS),
        '\u{27C0}'..='\u{27EF}' => Some(MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A),
        '\u{27F0}'..='\u{27FF}' => Some(SUPPLEMENTAL_ARROWS_A),
        '\u{2800}'..='\u{28FF}' => Some(BRAILLE_PATTERNS),
        '\u{2900}'..='\u{297F}' => Some(SUPPLEMENTAL_ARROWS_B),
        '\u{2980}'..='\u{29FF}' => Some(MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B),
        '\u{2A00}'..='\u{2AFF}' => Some(SUPPLEMENTAL_MATHEMATICAL_OPERATORS),
        '\u{2B00}'..='\u{2BFF}' => Some(MISCELLANEOUS_SYMBOLS_AND_ARROWS),
        '\u{2C00}'..='\u{2C5F}' => Some(GLAGOLITIC),
        '\u{2C60}'..='\u{2C7F}' => Some(LATIN_EXTENDED_C),
        '\u{2C80}'..='\u{2CFF}' => Some(COPTIC),
        '\u{2D00}'..='\u{2D2F}' => Some(GEORGIAN_SUPPLEMENT),
        '\u{2D30}'..='\u{2D7F}' => Some(TIFINAGH),
        '\u{2D80}'..='\u{2DDF}' => Some(ETHIOPIC_EXTENDED),
        '\u{2DE0}'..='\u{2DFF}' => Some(CYRILLIC_EXTENDED_A),
        '\u{2E00}'..='\u{2E7F}' => Some(SUPPLEMENTAL_PUNCTUATION),
        '\u{2E80}'..='\u{2EFF}' => Some(CJK_RADICALS_SUPPLEMENT),
        '\u{2F00}'..='\u{2FDF}' => Some(KANGXI_RADICALS),
        '\u{2FF0}'..='\u{2FFF}' => Some(IDEOGRAPHIC_DESCRIPTION_CHARACTERS),
        '\u{3000}'..='\u{303F}' => Some(CJK_SYMBOLS_AND_PUNCTUATION),
        '\u{3040}'..='\u{309F}' => Some(HIRAGANA),
        '\u{30A0}'..='\u{30FF}' => Some(KATAKANA),
        '\u{3100}'..='\u{312F}' => Some(BOPOMOFO),
        '\u{3130}'..='\u{318F}' => Some(HANGUL_COMPATIBILITY_JAMO),
        '\u{3190}'..='\u{319F}' => Some(KANBUN),
        '\u{31A0}'..='\u{31BF}' => Some(BOPOMOFO_EXTENDED),
        '\u{31C0}'..='\u{31EF}' => Some(CJK_STROKES),
        '\u{31F0}'..='\u{31FF}' => Some(KATAKANA_PHONETIC_EXTENSIONS),
        '\u{3200}'..='\u{32FF}' => Some(ENCLOSED_CJK_LETTERS_AND_MONTHS),
        '\u{3300}'..='\u{33FF}' => Some(CJK_COMPATIBILITY),
        '\u{3400}'..='\u{4DBF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A),
        '\u{4DC0}'..='\u{4DFF}' => Some(YIJING_HEXAGRAM_SYMBOLS),
        '\u{4E00}'..='\u{9FFF}' => Some(CJK_UNIFIED_IDEOGRAPHS),
        '\u{A000}'..='\u{A48F}' => Some(YI_SYLLABLES),
        '\u{A490}'..='\u{A4CF}' => Some(YI_RADICALS),
        '\u{A4D0}'..='\u{A4FF}' => Some(LISU),
        '\u{A500}'..='\u{A63F}' => Some(VAI),
        '\u{A640}'..='\u{A69F}' => Some(CYRILLIC_EXTENDED_B),
        '\u{A6A0}'..='\u{A6FF}' => Some(BAMUM),
        '\u{A700}'..='\u{A71F}' => Some(MODIFIER_TONE_LETTERS),
        '\u{A720}'..='\u{A7FF}' => Some(LATIN_EXTENDED_D),
        '\u{A800}'..='\u{A82F}' => Some(SYLOTI_NAGRI),
        '\u{A830}'..='\u{A83F}' => Some(COMMON_INDIC_NUMBER_FORMS),
        '\u{A840}'..='\u{A87F}' => Some(PHAGS_PA),
        '\u{A880}'..='\u{A8DF}' => Some(SAURASHTRA),
        '\u{A8E0}'..='\u{A8FF}' => Some(DEVANAGARI_EXTENDED),
        '\u{A900}'..='\u{A92F}' => Some(KAYAH_LI),
        '\u{A930}'..='\u{A95F}' => Some(REJANG),
        '\u{A960}'..='\u{A97F}' => Some(HANGUL_JAMO_EXTENDED_A),
        '\u{A980}'..='\u{A9DF}' => Some(JAVANESE),
        '\u{A9E0}'..='\u{A9FF}' => Some(MYANMAR_EXTENDED_B),
        '\u{AA00}'..='\u{AA5F}' => Some(CHAM),
        '\u{AA60}'..='\u{AA7F}' => Some(MYANMAR_EXTENDED_A),
        '\u{AA80}'..='\u{AADF}' => Some(TAI_VIET),
        '\u{AAE0}'..='\u{AAFF}' => Some(MEETEI_MAYEK_EXTENSIONS),
        '\u{AB00}'..='\u{AB2F}' => Some(ETHIOPIC_EXTENDED_A),
        '\u{AB30}'..='\u{AB6F}' => Some(LATIN_EXTENDED_E),
        '\u{AB70}'..='\u{ABBF}' => Some(CHEROKEE_SUPPLEMENT),
        '\u{ABC0}'..='\u{ABFF}' => Some(MEETEI_MAYEK),
        '\u{AC00}'..='\u{D7AF}' => Some(HANGUL_SYLLABLES),
        '\u{D7B0}'..='\u{D7FF}' => Some(HANGUL_JAMO_EXTENDED_B),
        '\u{E000}'..='\u{F8FF}' => Some(PRIVATE_USE_AREA),
        '\u{F900}'..='\u{FAFF}' => Some(CJK_COMPATIBILITY_IDEOGRAPHS),
        '\u{FB00}'..='\u{FB4F}' => Some(ALPHABETIC_PRESENTATION_FORMS),
        '\u{FB50}'..='\u{FDFF}' => Some(ARABIC_PRESENTATION_FORMS_A),
        '\u{FE00}'..='\u{FE0F}' => Some(VARIATION_SELECTORS),
        '\u{FE10}'..='\u{FE1F}' => Some(VERTICAL_FORMS),
        '\u{FE20}'..='\u{FE2F}' => Some(COMBINING_HALF_MARKS),
        '\u{FE30}'..='\u{FE4F}' => Some(CJK_COMPATIBILITY_FORMS),
        '\u{FE50}'..='\u{FE6F}' => Some(SMALL_FORM_VARIANTS),
        '\u{FE70}'..='\u{FEFF}' => Some(ARABIC_PRESENTATION_FORMS_B),
        '\u{FF00}'..='\u{FFEF}' => Some(HALFWIDTH_AND_FULLWIDTH_FORMS),
        '\u{FFF0}'..='\u{FFFF}' => Some(SPECIALS),
        '\u{10000}'..='\u{1007F}' => Some(LINEAR_B_SYLLABARY),
        '\u{10080}'..='\u{100FF}' => Some(LINEAR_B_IDEOGRAMS),
        '\u{10100}'..='\u{1013F}' => Some(AEGEAN_NUMBERS),
        '\u{10140}'..='\u{1018F}' => Some(ANCIENT_GREEK_NUMBERS),
        '\u{10190}'..='\u{101CF}' => Some(ANCIENT_SYMBOLS),
        '\u{101D0}'..='\u{101FF}' => Some(PHAISTOS_DISC),
        '\u{10280}'..='\u{1029F}' => Some(LYCIAN),
        '\u{102A0}'..='\u{102DF}' => Some(CARIAN),
        '\u{102E0}'..='\u{102FF}' => Some(COPTIC_EPACT_NUMBERS),
        '\u{10300}'..='\u{1032F}' => Some(OLD_ITALIC),
        '\u{10330}'..='\u{1034F}' => Some(GOTHIC),
        '\u{10350}'..='\u{1037F}' => Some(OLD_PERMIC),
        '\u{10380}'..='\u{1039F}' => Some(UGARITIC),
        '\u{103A0}'..='\u{103DF}' => Some(OLD_PERSIAN),
        '\u{10400}'..='\u{1044F}' => Some(DESERET),
        '\u{10450}'..='\u{1047F}' => Some(SHAVIAN),
        '\u{10480}'..='\u{104AF}' => Some(OSMANYA),
        '\u{104B0}'..='\u{104FF}' => Some(OSAGE),
        '\u{10500}'..='\u{1052F}' => Some(ELBASAN),
        '\u{10530}'..='\u{1056F}' => Some(CAUCASIAN_ALBANIAN),
        '\u{10570}'..='\u{105BF}' => Some(VITHKUQI),
        '\u{105C0}'..='\u{105FF}' => Some(TODHRI),
        '\u{10600}'..='\u{1077F}' => Some(LINEAR_A),
        '\u{10780}'..='\u{107BF}' => Some(LATIN_EXTENDED_F),
        '\u{10800}'..='\u{1083F}' => Some(CYPRIOT_SYLLABARY),
        '\u{10840}'..='\u{1085F}' => Some(IMPERIAL_ARAMAIC),
        '\u{10860}'..='\u{1087F}' => Some(PALMYRENE),
        '\u{10880}'..='\u{108AF}' => Some(NABATAEAN),
        '\u{108E0}'..='\u{108FF}' => Some(HATRAN),
        '\u{10900}'..='\u{1091F}' => Some(PHOENICIAN),
        '\u{10920}'..='\u{1093F}' => Some(LYDIAN),
        '\u{10980}'..='\u{1099F}' => Some(MEROITIC_HIEROGLYPHS),
        '\u{109A0}'..='\u{109FF}' => Some(MEROITIC_CURSIVE),
        '\u{10A00}'..='\u{10A5F}' => Some(KHAROSHTHI),
        '\u{10A60}'..='\u{10A7F}' => Some(OLD_SOUTH_ARABIAN),
        '\u{10A80}'..='\u{10A9F}' => Some(OLD_NORTH_ARABIAN),
        '\u{10AC0}'..='\u{10AFF}' => Some(MANICHAEAN),
        '\u{10B00}'..='\u{10B3F}' => Some(AVESTAN),
        '\u{10B40}'..='\u{10B5F}' => Some(INSCRIPTIONAL_PARTHIAN),
        '\u{10B60}'..='\u{10B7F}' => Some(INSCRIPTIONAL_PAHLAVI),
        '\u{10B80}'..='\u{10BAF}' => Some(PSALTER_PAHLAVI),
        '\u{10C00}'..='\u{10C4F}' => Some(OLD_TURKIC),
        '\u{10C80}'..='\u{10CFF}' => Some(OLD_HUNGARIAN),
        '\u{10D00}'..='\u{10D3F}' => Some(HANIFI_ROHINGYA),
        '\u{10D40}'..='\u{10D8F}' => Some(GARAY),
        '\u{10E60}'..='\u{10E7F}' => Some(RUMI_NUMERAL_SYMBOLS),
        '\u{10E80}'..='\u{10EBF}' => Some(YEZIDI),
        '\u{10EC0}'..='\u{10EFF}' => Some(ARABIC_EXTENDED_C),
        '\u{10F00}'..='\u{10F2F}' => Some(OLD_SOGDIAN),
        '\u{10F30}'..='\u{10F6F}' => Some(SOGDIAN),
        '\u{10F70}'..='\u{10FAF}' => Some(OLD_UYGHUR),
        '\u{10FB0}'..='\u{10FDF}' => Some(CHORASMIAN),
        '\u{10FE0}'..='\u{10FFF}' => Some(ELYMAIC),
        '\u{11000}'..='\u{1107F}' => Some(BRAHMI),
        '\u{11080}'..='\u{110CF}' => Some(KAITHI),
        '\u{110D0}'..='\u{110FF}' => Some(SORA_SOMPENG),
        '\u{11100}'..='\u{1114F}' => Some(CHAKMA),
        '\u{11150}'..='\u{1117F}' => Some(MAHAJANI),
        '\u{11180}'..='\u{111DF}' => Some(SHARADA),
        '\u{111E0}'..='\u{111FF}' => Some(SINHALA_ARCHAIC_NUMBERS),
        '\u{11200}'..='\u{1124F}' => Some(KHOJKI),
        '\u{11280}'..='\u{112AF}' => Some(MULTANI),
        '\u{112B0}'..='\u{112FF}' => Some(KHUDAWADI),
        '\u{11300}'..='\u{1137F}' => Some(GRANTHA),
        '\u{11380}'..='\u{113FF}' => Some(TULU_TIGALARI),
        '\u{11400}'..='\u{1147F}' => Some(NEWA),
        '\u{11480}'..='\u{114DF}' => Some(TIRHUTA),
        '\u{11580}'..='\u{115FF}' => Some(SIDDHAM),
        '\u{11600}'..='\u{1165F}' => Some(MODI),
        '\u{11660}'..='\u{1167F}' => Some(MONGOLIAN_SUPPLEMENT),
        '\u{11680}'..='\u{116CF}' => Some(TAKRI),
        '\u{116D0}'..='\u{116FF}' => Some(MYANMAR_EXTENDED_C),
        '\u{11700}'..='\u{1174F}' => Some(AHOM),
        '\u{11800}'..='\u{1184F}' => Some(DOGRA),
        '\u{118A0}'..='\u{118FF}' => Some(WARANG_CITI),
        '\u{11900}'..='\u{1195F}' => Some(DIVES_AKURU),
        '\u{119A0}'..='\u{119FF}' => Some(NANDINAGARI),
        '\u{11A00}'..='\u{11A4F}' => Some(ZANABAZAR_SQUARE),
        '\u{11A50}'..='\u{11AAF}' => Some(SOYOMBO),
        '\u{11AB0}'..='\u{11ABF}' => Some(UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A),
        '\u{11AC0}'..='\u{11AFF}' => Some(PAU_CIN_HAU),
        '\u{11B00}'..='\u{11B5F}' => Some(DEVANAGARI_EXTENDED_A),
        '\u{11BC0}'..='\u{11BFF}' => Some(SUNUWAR),
        '\u{11C00}'..='\u{11C6F}' => Some(BHAIKSUKI),
        '\u{11C70}'..='\u{11CBF}' => Some(MARCHEN),
        '\u{11D00}'..='\u{11D5F}' => Some(MASARAM_GONDI),
        '\u{11D60}'..='\u{11DAF}' => Some(GUNJALA_GONDI),
        '\u{11EE0}'..='\u{11EFF}' => Some(MAKASAR),
        '\u{11F00}'..='\u{11F5F}' => Some(KAWI),
        '\u{11FB0}'..='\u{11FBF}' => Some(LISU_SUPPLEMENT),
        '\u{11FC0}'..='\u{11FFF}' => Some(TAMIL_SUPPLEMENT),
        '\u{12000}'..='\u{123FF}' => Some(CUNEIFORM),
        '\u{12400}'..='\u{1247F}' => Some(CUNEIFORM_NUMBERS_AND_PUNCTUATION),
        '\u{12480}'..='\u{1254F}' => Some(EARLY_DYNASTIC_CUNEIFORM),
        '\u{12F90}'..='\u{12FFF}' => Some(CYPRO_MINOAN),
        '\u{13000}'..='\u{1342F}' => Some(EGYPTIAN_HIEROGLYPHS),
        '\u{13430}'..='\u{1345F}' => Some(EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS),
        '\u{13460}'..='\u{143FF}' => Some(EGYPTIAN_HIEROGLYPHS_EXTENDED_A),
        '\u{14400}'..='\u{1467F}' => Some(ANATOLIAN_HIEROGLYPHS),
        '\u{16100}'..='\u{1613F}' => Some(GURUNG_KHEMA),
        '\u{16800}'..='\u{16A3F}' => Some(BAMUM_SUPPLEMENT),
        '\u{16A40}'..='\u{16A6F}' => Some(MRO),
        '\u{16A70}'..='\u{16ACF}' => Some(TANGSA),
        '\u{16AD0}'..='\u{16AFF}' => Some(BASSA_VAH),
        '\u{16B00}'..='\u{16B8F}' => Some(PAHAWH_HMONG),
        '\u{16D40}'..='\u{16D7F}' => Some(KIRAT_RAI),
        '\u{16E40}'..='\u{16E9F}' => Some(MEDEFAIDRIN),
        '\u{16F00}'..='\u{16F9F}' => Some(MIAO),
        '\u{16FE0}'..='\u{16FFF}' => Some(IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION),
        '\u{17000}'..='\u{187FF}' => Some(TANGUT),
        '\u{18800}'..='\u{18AFF}' => Some(TANGUT_COMPONENTS),
        '\u{18B00}'..='\u{18CFF}' => Some(KHITAN_SMALL_SCRIPT),
        '\u{18D00}'..='\u{18D7F}' => Some(TANGUT_SUPPLEMENT),
        '\u{1AFF0}'..='\u{1AFFF}' => Some(KANA_EXTENDED_B),
        '\u{1B000}'..='\u{1B0FF}' => Some(KANA_SUPPLEMENT),
        '\u{1B100}'..='\u{1B12F}' => Some(KANA_EXTENDED_A),
        '\u{1B130}'..='\u{1B16F}' => Some(SMALL_KANA_EXTENSION),
        '\u{1B170}'..='\u{1B2FF}' => Some(NUSHU),
        '\u{1BC00}'..='\u{1BC9F}' => Some(DUPLOYAN),
        '\u{1BCA0}'..='\u{1BCAF}' => Some(SHORTHAND_FORMAT_CONTROLS),
        '\u{1CC00}'..='\u{1CEBF}' => Some(SYMBOLS_FOR_LEGACY_COMPUTING_SUPPLEMENT),
        '\u{1CF00}'..='\u{1CFCF}' => Some(ZNAMENNY_MUSICAL_NOTATION),
        '\u{1D000}'..='\u{1D0FF}' => Some(BYZANTINE_MUSICAL_SYMBOLS),
        '\u{1D100}'..='\u{1D1FF}' => Some(MUSICAL_SYMBOLS),
        '\u{1D200}'..='\u{1D24F}' => Some(ANCIENT_GREEK_MUSICAL_NOTATION),
        '\u{1D2C0}'..='\u{1D2DF}' => Some(KAKTOVIK_NUMERALS),
        '\u{1D2E0}'..='\u{1D2FF}' => Some(MAYAN_NUMERALS),
        '\u{1D300}'..='\u{1D35F}' => Some(TAI_XUAN_JING_SYMBOLS),
        '\u{1D360}'..='\u{1D37F}' => Some(COUNTING_ROD_NUMERALS),
        '\u{1D400}'..='\u{1D7FF}' => Some(MATHEMATICAL_ALPHANUMERIC_SYMBOLS),
        '\u{1D800}'..='\u{1DAAF}' => Some(SUTTON_SIGNWRITING),
        '\u{1DF00}'..='\u{1DFFF}' => Some(LATIN_EXTENDED_G),
        '\u{1E000}'..='\u{1E02F}' => Some(GLAGOLITIC_SUPPLEMENT),
        '\u{1E030}'..='\u{1E08F}' => Some(CYRILLIC_EXTENDED_D),
        '\u{1E100}'..='\u{1E14F}' => Some(NYIAKENG_PUACHUE_HMONG),
        '\u{1E290}'..='\u{1E2BF}' => Some(TOTO),
        '\u{1E2C0}'..='\u{1E2FF}' => Some(WANCHO),
        '\u{1E4D0}'..='\u{1E4FF}' => Some(NAG_MUNDARI),
        '\u{1E5D0}'..='\u{1E5FF}' => Some(OL_ONAL),
        '\u{1E7E0}'..='\u{1E7FF}' => Some(ETHIOPIC_EXTENDED_B),
        '\u{1E800}'..='\u{1E8DF}' => Some(MENDE_KIKAKUI),
        '\u{1E900}'..='\u{1E95F}' => Some(ADLAM),
        '\u{1EC70}'..='\u{1ECBF}' => Some(INDIC_SIYAQ_NUMBERS),
        '\u{1ED00}'..='\u{1ED4F}' => Some(OTTOMAN_SIYAQ_NUMBERS),
        '\u{1EE00}'..='\u{1EEFF}' => Some(ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS),
        '\u{1F000}'..='\u{1F02F}' => Some(MAHJONG_TILES),
        '\u{1F030}'..='\u{1F09F}' => Some(DOMINO_TILES),
        '\u{1F0A0}'..='\u{1F0FF}' => Some(PLAYING_CARDS),
        '\u{1F100}'..='\u{1F1FF}' => Some(ENCLOSED_ALPHANUMERIC_SUPPLEMENT),
        '\u{1F200}'..='\u{1F2FF}' => Some(ENCLOSED_IDEOGRAPHIC_SUPPLEMENT),
        '\u{1F300}'..='\u{1F5FF}' => Some(MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS),
        '\u{1F600}'..='\u{1F64F}' => Some(EMOTICONS),
        '\u{1F650}'..='\u{1F67F}' => Some(ORNAMENTAL_DINGBATS),
        '\u{1F680}'..='\u{1F6FF}' => Some(TRANSPORT_AND_MAP_SYMBOLS),
        '\u{1F700}'..='\u{1F77F}' => Some(ALCHEMICAL_SYMBOLS),
        '\u{1F780}'..='\u{1F7FF}' => Some(GEOMETRIC_SHAPES_EXTENDED),
        '\u{1F800}'..='\u{1F8FF}' => Some(SUPPLEMENTAL_ARROWS_C),
        '\u{1F900}'..='\u{1F9FF}' => Some(SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS),
        '\u{1FA00}'..='\u{1FA6F}' => Some(CHESS_SYMBOLS),
        '\u{1FA70}'..='\u{1FAFF}' => Some(SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A),
        '\u{1FB00}'..='\u{1FBFF}' => Some(SYMBOLS_FOR_LEGACY_COMPUTING),
        '\u{20000}'..='\u{2A6DF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B),
        '\u{2A700}'..='\u{2B73F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C),
        '\u{2B740}'..='\u{2B81F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D),
        '\u{2B820}'..='\u{2CEAF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E),
        '\u{2CEB0}'..='\u{2EBEF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F),
        '\u{2EBF0}'..='\u{2EE5F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_I),
        '\u{2F800}'..='\u{2FA1F}' => Some(CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT),
        '\u{30000}'..='\u{3134F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G),
        '\u{31350}'..='\u{323AF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H),
        '\u{E0000}'..='\u{E007F}' => Some(TAGS),
        '\u{E0100}'..='\u{E01EF}' => Some(VARIATION_SELECTORS_SUPPLEMENT),
        '\u{F0000}'..='\u{FFFFF}' => Some(SUPPLEMENTARY_PRIVATE_USE_AREA_A),
        '\u{100000}'..='\u{10FFFF}' => Some(SUPPLEMENTARY_PRIVATE_USE_AREA_B),
        _ => None,
    }
}
