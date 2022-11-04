// The dataset is from https://www.unicode.org/Public/UNIDATA/Blocks.txt

use crate::UnicodeBlock;

pub const BASIC_LATIN: UnicodeBlock = UnicodeBlock {
    name: "Basic Latin",
    start: 0,
    end: 127,
};
pub const LATIN_1_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Latin-1 Supplement",
    start: 128,
    end: 255,
};
pub const LATIN_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-A",
    start: 256,
    end: 383,
};
pub const LATIN_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-B",
    start: 384,
    end: 591,
};
pub const IPA_EXTENSIONS: UnicodeBlock = UnicodeBlock {
    name: "IPA Extensions",
    start: 592,
    end: 687,
};
pub const SPACING_MODIFIER_LETTERS: UnicodeBlock = UnicodeBlock {
    name: "Spacing Modifier Letters",
    start: 688,
    end: 767,
};
pub const COMBINING_DIACRITICAL_MARKS: UnicodeBlock = UnicodeBlock {
    name: "Combining Diacritical Marks",
    start: 768,
    end: 879,
};
pub const GREEK_AND_COPTIC: UnicodeBlock = UnicodeBlock {
    name: "Greek and Coptic",
    start: 880,
    end: 1023,
};
pub const CYRILLIC: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic",
    start: 1024,
    end: 1279,
};
pub const CYRILLIC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic Supplement",
    start: 1280,
    end: 1327,
};
pub const ARMENIAN: UnicodeBlock = UnicodeBlock {
    name: "Armenian",
    start: 1328,
    end: 1423,
};
pub const HEBREW: UnicodeBlock = UnicodeBlock {
    name: "Hebrew",
    start: 1424,
    end: 1535,
};
pub const ARABIC: UnicodeBlock = UnicodeBlock {
    name: "Arabic",
    start: 1536,
    end: 1791,
};
pub const SYRIAC: UnicodeBlock = UnicodeBlock {
    name: "Syriac",
    start: 1792,
    end: 1871,
};
pub const ARABIC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Arabic Supplement",
    start: 1872,
    end: 1919,
};
pub const THAANA: UnicodeBlock = UnicodeBlock {
    name: "Thaana",
    start: 1920,
    end: 1983,
};
pub const NKO: UnicodeBlock = UnicodeBlock {
    name: "NKo",
    start: 1984,
    end: 2047,
};
pub const SAMARITAN: UnicodeBlock = UnicodeBlock {
    name: "Samaritan",
    start: 2048,
    end: 2111,
};
pub const MANDAIC: UnicodeBlock = UnicodeBlock {
    name: "Mandaic",
    start: 2112,
    end: 2143,
};
pub const SYRIAC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Syriac Supplement",
    start: 2144,
    end: 2159,
};
pub const ARABIC_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Arabic Extended-B",
    start: 2160,
    end: 2207,
};
pub const ARABIC_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Arabic Extended-A",
    start: 2208,
    end: 2303,
};
pub const DEVANAGARI: UnicodeBlock = UnicodeBlock {
    name: "Devanagari",
    start: 2304,
    end: 2431,
};
pub const BENGALI: UnicodeBlock = UnicodeBlock {
    name: "Bengali",
    start: 2432,
    end: 2559,
};
pub const GURMUKHI: UnicodeBlock = UnicodeBlock {
    name: "Gurmukhi",
    start: 2560,
    end: 2687,
};
pub const GUJARATI: UnicodeBlock = UnicodeBlock {
    name: "Gujarati",
    start: 2688,
    end: 2815,
};
pub const ORIYA: UnicodeBlock = UnicodeBlock {
    name: "Oriya",
    start: 2816,
    end: 2943,
};
pub const TAMIL: UnicodeBlock = UnicodeBlock {
    name: "Tamil",
    start: 2944,
    end: 3071,
};
pub const TELUGU: UnicodeBlock = UnicodeBlock {
    name: "Telugu",
    start: 3072,
    end: 3199,
};
pub const KANNADA: UnicodeBlock = UnicodeBlock {
    name: "Kannada",
    start: 3200,
    end: 3327,
};
pub const MALAYALAM: UnicodeBlock = UnicodeBlock {
    name: "Malayalam",
    start: 3328,
    end: 3455,
};
pub const SINHALA: UnicodeBlock = UnicodeBlock {
    name: "Sinhala",
    start: 3456,
    end: 3583,
};
pub const THAI: UnicodeBlock = UnicodeBlock {
    name: "Thai",
    start: 3584,
    end: 3711,
};
pub const LAO: UnicodeBlock = UnicodeBlock {
    name: "Lao",
    start: 3712,
    end: 3839,
};
pub const TIBETAN: UnicodeBlock = UnicodeBlock {
    name: "Tibetan",
    start: 3840,
    end: 4095,
};
pub const MYANMAR: UnicodeBlock = UnicodeBlock {
    name: "Myanmar",
    start: 4096,
    end: 4255,
};
pub const GEORGIAN: UnicodeBlock = UnicodeBlock {
    name: "Georgian",
    start: 4256,
    end: 4351,
};
pub const HANGUL_JAMO: UnicodeBlock = UnicodeBlock {
    name: "Hangul Jamo",
    start: 4352,
    end: 4607,
};
pub const ETHIOPIC: UnicodeBlock = UnicodeBlock {
    name: "Ethiopic",
    start: 4608,
    end: 4991,
};
pub const ETHIOPIC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Ethiopic Supplement",
    start: 4992,
    end: 5023,
};
pub const CHEROKEE: UnicodeBlock = UnicodeBlock {
    name: "Cherokee",
    start: 5024,
    end: 5119,
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UnicodeBlock = UnicodeBlock {
    name: "Unified Canadian Aboriginal Syllabics",
    start: 5120,
    end: 5759,
};
pub const OGHAM: UnicodeBlock = UnicodeBlock {
    name: "Ogham",
    start: 5760,
    end: 5791,
};
pub const RUNIC: UnicodeBlock = UnicodeBlock {
    name: "Runic",
    start: 5792,
    end: 5887,
};
pub const TAGALOG: UnicodeBlock = UnicodeBlock {
    name: "Tagalog",
    start: 5888,
    end: 5919,
};
pub const HANUNOO: UnicodeBlock = UnicodeBlock {
    name: "Hanunoo",
    start: 5920,
    end: 5951,
};
pub const BUHID: UnicodeBlock = UnicodeBlock {
    name: "Buhid",
    start: 5952,
    end: 5983,
};
pub const TAGBANWA: UnicodeBlock = UnicodeBlock {
    name: "Tagbanwa",
    start: 5984,
    end: 6015,
};
pub const KHMER: UnicodeBlock = UnicodeBlock {
    name: "Khmer",
    start: 6016,
    end: 6143,
};
pub const MONGOLIAN: UnicodeBlock = UnicodeBlock {
    name: "Mongolian",
    start: 6144,
    end: 6319,
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Unified Canadian Aboriginal Syllabics Extended",
    start: 6320,
    end: 6399,
};
pub const LIMBU: UnicodeBlock = UnicodeBlock {
    name: "Limbu",
    start: 6400,
    end: 6479,
};
pub const TAI_LE: UnicodeBlock = UnicodeBlock {
    name: "Tai Le",
    start: 6480,
    end: 6527,
};
pub const NEW_TAI_LUE: UnicodeBlock = UnicodeBlock {
    name: "New Tai Lue",
    start: 6528,
    end: 6623,
};
pub const KHMER_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Khmer Symbols",
    start: 6624,
    end: 6655,
};
pub const BUGINESE: UnicodeBlock = UnicodeBlock {
    name: "Buginese",
    start: 6656,
    end: 6687,
};
pub const TAI_THAM: UnicodeBlock = UnicodeBlock {
    name: "Tai Tham",
    start: 6688,
    end: 6831,
};
pub const COMBINING_DIACRITICAL_MARKS_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Combining Diacritical Marks Extended",
    start: 6832,
    end: 6911,
};
pub const BALINESE: UnicodeBlock = UnicodeBlock {
    name: "Balinese",
    start: 6912,
    end: 7039,
};
pub const SUNDANESE: UnicodeBlock = UnicodeBlock {
    name: "Sundanese",
    start: 7040,
    end: 7103,
};
pub const BATAK: UnicodeBlock = UnicodeBlock {
    name: "Batak",
    start: 7104,
    end: 7167,
};
pub const LEPCHA: UnicodeBlock = UnicodeBlock {
    name: "Lepcha",
    start: 7168,
    end: 7247,
};
pub const OL_CHIKI: UnicodeBlock = UnicodeBlock {
    name: "Ol Chiki",
    start: 7248,
    end: 7295,
};
pub const CYRILLIC_EXTENDED_C: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic Extended-C",
    start: 7296,
    end: 7311,
};
pub const GEORGIAN_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Georgian Extended",
    start: 7312,
    end: 7359,
};
pub const SUNDANESE_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Sundanese Supplement",
    start: 7360,
    end: 7375,
};
pub const VEDIC_EXTENSIONS: UnicodeBlock = UnicodeBlock {
    name: "Vedic Extensions",
    start: 7376,
    end: 7423,
};
pub const PHONETIC_EXTENSIONS: UnicodeBlock = UnicodeBlock {
    name: "Phonetic Extensions",
    start: 7424,
    end: 7551,
};
pub const PHONETIC_EXTENSIONS_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Phonetic Extensions Supplement",
    start: 7552,
    end: 7615,
};
pub const COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Combining Diacritical Marks Supplement",
    start: 7616,
    end: 7679,
};
pub const LATIN_EXTENDED_ADDITIONAL: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended Additional",
    start: 7680,
    end: 7935,
};
pub const GREEK_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Greek Extended",
    start: 7936,
    end: 8191,
};
pub const GENERAL_PUNCTUATION: UnicodeBlock = UnicodeBlock {
    name: "General Punctuation",
    start: 8192,
    end: 8303,
};
pub const SUPERSCRIPTS_AND_SUBSCRIPTS: UnicodeBlock = UnicodeBlock {
    name: "Superscripts and Subscripts",
    start: 8304,
    end: 8351,
};
pub const CURRENCY_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Currency Symbols",
    start: 8352,
    end: 8399,
};
pub const COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Combining Diacritical Marks for Symbols",
    start: 8400,
    end: 8447,
};
pub const LETTERLIKE_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Letterlike Symbols",
    start: 8448,
    end: 8527,
};
pub const NUMBER_FORMS: UnicodeBlock = UnicodeBlock {
    name: "Number Forms",
    start: 8528,
    end: 8591,
};
pub const ARROWS: UnicodeBlock = UnicodeBlock {
    name: "Arrows",
    start: 8592,
    end: 8703,
};
pub const MATHEMATICAL_OPERATORS: UnicodeBlock = UnicodeBlock {
    name: "Mathematical Operators",
    start: 8704,
    end: 8959,
};
pub const MISCELLANEOUS_TECHNICAL: UnicodeBlock = UnicodeBlock {
    name: "Miscellaneous Technical",
    start: 8960,
    end: 9215,
};
pub const CONTROL_PICTURES: UnicodeBlock = UnicodeBlock {
    name: "Control Pictures",
    start: 9216,
    end: 9279,
};
pub const OPTICAL_CHARACTER_RECOGNITION: UnicodeBlock = UnicodeBlock {
    name: "Optical Character Recognition",
    start: 9280,
    end: 9311,
};
pub const ENCLOSED_ALPHANUMERICS: UnicodeBlock = UnicodeBlock {
    name: "Enclosed Alphanumerics",
    start: 9312,
    end: 9471,
};
pub const BOX_DRAWING: UnicodeBlock = UnicodeBlock {
    name: "Box Drawing",
    start: 9472,
    end: 9599,
};
pub const BLOCK_ELEMENTS: UnicodeBlock = UnicodeBlock {
    name: "Block Elements",
    start: 9600,
    end: 9631,
};
pub const GEOMETRIC_SHAPES: UnicodeBlock = UnicodeBlock {
    name: "Geometric Shapes",
    start: 9632,
    end: 9727,
};
pub const MISCELLANEOUS_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Miscellaneous Symbols",
    start: 9728,
    end: 9983,
};
pub const DINGBATS: UnicodeBlock = UnicodeBlock {
    name: "Dingbats",
    start: 9984,
    end: 10175,
};
pub const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UnicodeBlock = UnicodeBlock {
    name: "Miscellaneous Mathematical Symbols-A",
    start: 10176,
    end: 10223,
};
pub const SUPPLEMENTAL_ARROWS_A: UnicodeBlock = UnicodeBlock {
    name: "Supplemental Arrows-A",
    start: 10224,
    end: 10239,
};
pub const BRAILLE_PATTERNS: UnicodeBlock = UnicodeBlock {
    name: "Braille Patterns",
    start: 10240,
    end: 10495,
};
pub const SUPPLEMENTAL_ARROWS_B: UnicodeBlock = UnicodeBlock {
    name: "Supplemental Arrows-B",
    start: 10496,
    end: 10623,
};
pub const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UnicodeBlock = UnicodeBlock {
    name: "Miscellaneous Mathematical Symbols-B",
    start: 10624,
    end: 10751,
};
pub const SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UnicodeBlock = UnicodeBlock {
    name: "Supplemental Mathematical Operators",
    start: 10752,
    end: 11007,
};
pub const MISCELLANEOUS_SYMBOLS_AND_ARROWS: UnicodeBlock = UnicodeBlock {
    name: "Miscellaneous Symbols and Arrows",
    start: 11008,
    end: 11263,
};
pub const GLAGOLITIC: UnicodeBlock = UnicodeBlock {
    name: "Glagolitic",
    start: 11264,
    end: 11359,
};
pub const LATIN_EXTENDED_C: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-C",
    start: 11360,
    end: 11391,
};
pub const COPTIC: UnicodeBlock = UnicodeBlock {
    name: "Coptic",
    start: 11392,
    end: 11519,
};
pub const GEORGIAN_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Georgian Supplement",
    start: 11520,
    end: 11567,
};
pub const TIFINAGH: UnicodeBlock = UnicodeBlock {
    name: "Tifinagh",
    start: 11568,
    end: 11647,
};
pub const ETHIOPIC_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Ethiopic Extended",
    start: 11648,
    end: 11743,
};
pub const CYRILLIC_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic Extended-A",
    start: 11744,
    end: 11775,
};
pub const SUPPLEMENTAL_PUNCTUATION: UnicodeBlock = UnicodeBlock {
    name: "Supplemental Punctuation",
    start: 11776,
    end: 11903,
};
pub const CJK_RADICALS_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "CJK Radicals Supplement",
    start: 11904,
    end: 12031,
};
pub const KANGXI_RADICALS: UnicodeBlock = UnicodeBlock {
    name: "Kangxi Radicals",
    start: 12032,
    end: 12255,
};
pub const IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UnicodeBlock = UnicodeBlock {
    name: "Ideographic Description Characters",
    start: 12272,
    end: 12287,
};
pub const CJK_SYMBOLS_AND_PUNCTUATION: UnicodeBlock = UnicodeBlock {
    name: "CJK Symbols and Punctuation",
    start: 12288,
    end: 12351,
};
pub const HIRAGANA: UnicodeBlock = UnicodeBlock {
    name: "Hiragana",
    start: 12352,
    end: 12447,
};
pub const KATAKANA: UnicodeBlock = UnicodeBlock {
    name: "Katakana",
    start: 12448,
    end: 12543,
};
pub const BOPOMOFO: UnicodeBlock = UnicodeBlock {
    name: "Bopomofo",
    start: 12544,
    end: 12591,
};
pub const HANGUL_COMPATIBILITY_JAMO: UnicodeBlock = UnicodeBlock {
    name: "Hangul Compatibility Jamo",
    start: 12592,
    end: 12687,
};
pub const KANBUN: UnicodeBlock = UnicodeBlock {
    name: "Kanbun",
    start: 12688,
    end: 12703,
};
pub const BOPOMOFO_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Bopomofo Extended",
    start: 12704,
    end: 12735,
};
pub const CJK_STROKES: UnicodeBlock = UnicodeBlock {
    name: "CJK Strokes",
    start: 12736,
    end: 12783,
};
pub const KATAKANA_PHONETIC_EXTENSIONS: UnicodeBlock = UnicodeBlock {
    name: "Katakana Phonetic Extensions",
    start: 12784,
    end: 12799,
};
pub const ENCLOSED_CJK_LETTERS_AND_MONTHS: UnicodeBlock = UnicodeBlock {
    name: "Enclosed CJK Letters and Months",
    start: 12800,
    end: 13055,
};
pub const CJK_COMPATIBILITY: UnicodeBlock = UnicodeBlock {
    name: "CJK Compatibility",
    start: 13056,
    end: 13311,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension A",
    start: 13312,
    end: 19903,
};
pub const YIJING_HEXAGRAM_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Yijing Hexagram Symbols",
    start: 19904,
    end: 19967,
};
pub const CJK_UNIFIED_IDEOGRAPHS: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs",
    start: 19968,
    end: 40959,
};
pub const YI_SYLLABLES: UnicodeBlock = UnicodeBlock {
    name: "Yi Syllables",
    start: 40960,
    end: 42127,
};
pub const YI_RADICALS: UnicodeBlock = UnicodeBlock {
    name: "Yi Radicals",
    start: 42128,
    end: 42191,
};
pub const LISU: UnicodeBlock = UnicodeBlock {
    name: "Lisu",
    start: 42192,
    end: 42239,
};
pub const VAI: UnicodeBlock = UnicodeBlock {
    name: "Vai",
    start: 42240,
    end: 42559,
};
pub const CYRILLIC_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic Extended-B",
    start: 42560,
    end: 42655,
};
pub const BAMUM: UnicodeBlock = UnicodeBlock {
    name: "Bamum",
    start: 42656,
    end: 42751,
};
pub const MODIFIER_TONE_LETTERS: UnicodeBlock = UnicodeBlock {
    name: "Modifier Tone Letters",
    start: 42752,
    end: 42783,
};
pub const LATIN_EXTENDED_D: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-D",
    start: 42784,
    end: 43007,
};
pub const SYLOTI_NAGRI: UnicodeBlock = UnicodeBlock {
    name: "Syloti Nagri",
    start: 43008,
    end: 43055,
};
pub const COMMON_INDIC_NUMBER_FORMS: UnicodeBlock = UnicodeBlock {
    name: "Common Indic Number Forms",
    start: 43056,
    end: 43071,
};
pub const PHAGS_PA: UnicodeBlock = UnicodeBlock {
    name: "Phags-pa",
    start: 43072,
    end: 43135,
};
pub const SAURASHTRA: UnicodeBlock = UnicodeBlock {
    name: "Saurashtra",
    start: 43136,
    end: 43231,
};
pub const DEVANAGARI_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Devanagari Extended",
    start: 43232,
    end: 43263,
};
pub const KAYAH_LI: UnicodeBlock = UnicodeBlock {
    name: "Kayah Li",
    start: 43264,
    end: 43311,
};
pub const REJANG: UnicodeBlock = UnicodeBlock {
    name: "Rejang",
    start: 43312,
    end: 43359,
};
pub const HANGUL_JAMO_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Hangul Jamo Extended-A",
    start: 43360,
    end: 43391,
};
pub const JAVANESE: UnicodeBlock = UnicodeBlock {
    name: "Javanese",
    start: 43392,
    end: 43487,
};
pub const MYANMAR_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Myanmar Extended-B",
    start: 43488,
    end: 43519,
};
pub const CHAM: UnicodeBlock = UnicodeBlock {
    name: "Cham",
    start: 43520,
    end: 43615,
};
pub const MYANMAR_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Myanmar Extended-A",
    start: 43616,
    end: 43647,
};
pub const TAI_VIET: UnicodeBlock = UnicodeBlock {
    name: "Tai Viet",
    start: 43648,
    end: 43743,
};
pub const MEETEI_MAYEK_EXTENSIONS: UnicodeBlock = UnicodeBlock {
    name: "Meetei Mayek Extensions",
    start: 43744,
    end: 43775,
};
pub const ETHIOPIC_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Ethiopic Extended-A",
    start: 43776,
    end: 43823,
};
pub const LATIN_EXTENDED_E: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-E",
    start: 43824,
    end: 43887,
};
pub const CHEROKEE_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Cherokee Supplement",
    start: 43888,
    end: 43967,
};
pub const MEETEI_MAYEK: UnicodeBlock = UnicodeBlock {
    name: "Meetei Mayek",
    start: 43968,
    end: 44031,
};
pub const HANGUL_SYLLABLES: UnicodeBlock = UnicodeBlock {
    name: "Hangul Syllables",
    start: 44032,
    end: 55215,
};
pub const HANGUL_JAMO_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Hangul Jamo Extended-B",
    start: 55216,
    end: 55295,
};
pub const HIGH_SURROGATES: UnicodeBlock = UnicodeBlock {
    name: "High Surrogates",
    start: 55296,
    end: 56191,
};
pub const HIGH_PRIVATE_USE_SURROGATES: UnicodeBlock = UnicodeBlock {
    name: "High Private Use Surrogates",
    start: 56192,
    end: 56319,
};
pub const LOW_SURROGATES: UnicodeBlock = UnicodeBlock {
    name: "Low Surrogates",
    start: 56320,
    end: 57343,
};
pub const PRIVATE_USE_AREA: UnicodeBlock = UnicodeBlock {
    name: "Private Use Area",
    start: 57344,
    end: 63743,
};
pub const CJK_COMPATIBILITY_IDEOGRAPHS: UnicodeBlock = UnicodeBlock {
    name: "CJK Compatibility Ideographs",
    start: 63744,
    end: 64255,
};
pub const ALPHABETIC_PRESENTATION_FORMS: UnicodeBlock = UnicodeBlock {
    name: "Alphabetic Presentation Forms",
    start: 64256,
    end: 64335,
};
pub const ARABIC_PRESENTATION_FORMS_A: UnicodeBlock = UnicodeBlock {
    name: "Arabic Presentation Forms-A",
    start: 64336,
    end: 65023,
};
pub const VARIATION_SELECTORS: UnicodeBlock = UnicodeBlock {
    name: "Variation Selectors",
    start: 65024,
    end: 65039,
};
pub const VERTICAL_FORMS: UnicodeBlock = UnicodeBlock {
    name: "Vertical Forms",
    start: 65040,
    end: 65055,
};
pub const COMBINING_HALF_MARKS: UnicodeBlock = UnicodeBlock {
    name: "Combining Half Marks",
    start: 65056,
    end: 65071,
};
pub const CJK_COMPATIBILITY_FORMS: UnicodeBlock = UnicodeBlock {
    name: "CJK Compatibility Forms",
    start: 65072,
    end: 65103,
};
pub const SMALL_FORM_VARIANTS: UnicodeBlock = UnicodeBlock {
    name: "Small Form Variants",
    start: 65104,
    end: 65135,
};
pub const ARABIC_PRESENTATION_FORMS_B: UnicodeBlock = UnicodeBlock {
    name: "Arabic Presentation Forms-B",
    start: 65136,
    end: 65279,
};
pub const HALFWIDTH_AND_FULLWIDTH_FORMS: UnicodeBlock = UnicodeBlock {
    name: "Halfwidth and Fullwidth Forms",
    start: 65280,
    end: 65519,
};
pub const SPECIALS: UnicodeBlock = UnicodeBlock {
    name: "Specials",
    start: 65520,
    end: 65535,
};
pub const LINEAR_B_SYLLABARY: UnicodeBlock = UnicodeBlock {
    name: "Linear B Syllabary",
    start: 65536,
    end: 65663,
};
pub const LINEAR_B_IDEOGRAMS: UnicodeBlock = UnicodeBlock {
    name: "Linear B Ideograms",
    start: 65664,
    end: 65791,
};
pub const AEGEAN_NUMBERS: UnicodeBlock = UnicodeBlock {
    name: "Aegean Numbers",
    start: 65792,
    end: 65855,
};
pub const ANCIENT_GREEK_NUMBERS: UnicodeBlock = UnicodeBlock {
    name: "Ancient Greek Numbers",
    start: 65856,
    end: 65935,
};
pub const ANCIENT_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Ancient Symbols",
    start: 65936,
    end: 65999,
};
pub const PHAISTOS_DISC: UnicodeBlock = UnicodeBlock {
    name: "Phaistos Disc",
    start: 66000,
    end: 66047,
};
pub const LYCIAN: UnicodeBlock = UnicodeBlock {
    name: "Lycian",
    start: 66176,
    end: 66207,
};
pub const CARIAN: UnicodeBlock = UnicodeBlock {
    name: "Carian",
    start: 66208,
    end: 66271,
};
pub const COPTIC_EPACT_NUMBERS: UnicodeBlock = UnicodeBlock {
    name: "Coptic Epact Numbers",
    start: 66272,
    end: 66303,
};
pub const OLD_ITALIC: UnicodeBlock = UnicodeBlock {
    name: "Old Italic",
    start: 66304,
    end: 66351,
};
pub const GOTHIC: UnicodeBlock = UnicodeBlock {
    name: "Gothic",
    start: 66352,
    end: 66383,
};
pub const OLD_PERMIC: UnicodeBlock = UnicodeBlock {
    name: "Old Permic",
    start: 66384,
    end: 66431,
};
pub const UGARITIC: UnicodeBlock = UnicodeBlock {
    name: "Ugaritic",
    start: 66432,
    end: 66463,
};
pub const OLD_PERSIAN: UnicodeBlock = UnicodeBlock {
    name: "Old Persian",
    start: 66464,
    end: 66527,
};
pub const DESERET: UnicodeBlock = UnicodeBlock {
    name: "Deseret",
    start: 66560,
    end: 66639,
};
pub const SHAVIAN: UnicodeBlock = UnicodeBlock {
    name: "Shavian",
    start: 66640,
    end: 66687,
};
pub const OSMANYA: UnicodeBlock = UnicodeBlock {
    name: "Osmanya",
    start: 66688,
    end: 66735,
};
pub const OSAGE: UnicodeBlock = UnicodeBlock {
    name: "Osage",
    start: 66736,
    end: 66815,
};
pub const ELBASAN: UnicodeBlock = UnicodeBlock {
    name: "Elbasan",
    start: 66816,
    end: 66863,
};
pub const CAUCASIAN_ALBANIAN: UnicodeBlock = UnicodeBlock {
    name: "Caucasian Albanian",
    start: 66864,
    end: 66927,
};
pub const VITHKUQI: UnicodeBlock = UnicodeBlock {
    name: "Vithkuqi",
    start: 66928,
    end: 67007,
};
pub const LINEAR_A: UnicodeBlock = UnicodeBlock {
    name: "Linear A",
    start: 67072,
    end: 67455,
};
pub const LATIN_EXTENDED_F: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-F",
    start: 67456,
    end: 67519,
};
pub const CYPRIOT_SYLLABARY: UnicodeBlock = UnicodeBlock {
    name: "Cypriot Syllabary",
    start: 67584,
    end: 67647,
};
pub const IMPERIAL_ARAMAIC: UnicodeBlock = UnicodeBlock {
    name: "Imperial Aramaic",
    start: 67648,
    end: 67679,
};
pub const PALMYRENE: UnicodeBlock = UnicodeBlock {
    name: "Palmyrene",
    start: 67680,
    end: 67711,
};
pub const NABATAEAN: UnicodeBlock = UnicodeBlock {
    name: "Nabataean",
    start: 67712,
    end: 67759,
};
pub const HATRAN: UnicodeBlock = UnicodeBlock {
    name: "Hatran",
    start: 67808,
    end: 67839,
};
pub const PHOENICIAN: UnicodeBlock = UnicodeBlock {
    name: "Phoenician",
    start: 67840,
    end: 67871,
};
pub const LYDIAN: UnicodeBlock = UnicodeBlock {
    name: "Lydian",
    start: 67872,
    end: 67903,
};
pub const MEROITIC_HIEROGLYPHS: UnicodeBlock = UnicodeBlock {
    name: "Meroitic Hieroglyphs",
    start: 67968,
    end: 67999,
};
pub const MEROITIC_CURSIVE: UnicodeBlock = UnicodeBlock {
    name: "Meroitic Cursive",
    start: 68000,
    end: 68095,
};
pub const KHAROSHTHI: UnicodeBlock = UnicodeBlock {
    name: "Kharoshthi",
    start: 68096,
    end: 68191,
};
pub const OLD_SOUTH_ARABIAN: UnicodeBlock = UnicodeBlock {
    name: "Old South Arabian",
    start: 68192,
    end: 68223,
};
pub const OLD_NORTH_ARABIAN: UnicodeBlock = UnicodeBlock {
    name: "Old North Arabian",
    start: 68224,
    end: 68255,
};
pub const MANICHAEAN: UnicodeBlock = UnicodeBlock {
    name: "Manichaean",
    start: 68288,
    end: 68351,
};
pub const AVESTAN: UnicodeBlock = UnicodeBlock {
    name: "Avestan",
    start: 68352,
    end: 68415,
};
pub const INSCRIPTIONAL_PARTHIAN: UnicodeBlock = UnicodeBlock {
    name: "Inscriptional Parthian",
    start: 68416,
    end: 68447,
};
pub const INSCRIPTIONAL_PAHLAVI: UnicodeBlock = UnicodeBlock {
    name: "Inscriptional Pahlavi",
    start: 68448,
    end: 68479,
};
pub const PSALTER_PAHLAVI: UnicodeBlock = UnicodeBlock {
    name: "Psalter Pahlavi",
    start: 68480,
    end: 68527,
};
pub const OLD_TURKIC: UnicodeBlock = UnicodeBlock {
    name: "Old Turkic",
    start: 68608,
    end: 68687,
};
pub const OLD_HUNGARIAN: UnicodeBlock = UnicodeBlock {
    name: "Old Hungarian",
    start: 68736,
    end: 68863,
};
pub const HANIFI_ROHINGYA: UnicodeBlock = UnicodeBlock {
    name: "Hanifi Rohingya",
    start: 68864,
    end: 68927,
};
pub const RUMI_NUMERAL_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Rumi Numeral Symbols",
    start: 69216,
    end: 69247,
};
pub const YEZIDI: UnicodeBlock = UnicodeBlock {
    name: "Yezidi",
    start: 69248,
    end: 69311,
};
pub const ARABIC_EXTENDED_C: UnicodeBlock = UnicodeBlock {
    name: "Arabic Extended-C",
    start: 69312,
    end: 69375,
};
pub const OLD_SOGDIAN: UnicodeBlock = UnicodeBlock {
    name: "Old Sogdian",
    start: 69376,
    end: 69423,
};
pub const SOGDIAN: UnicodeBlock = UnicodeBlock {
    name: "Sogdian",
    start: 69424,
    end: 69487,
};
pub const OLD_UYGHUR: UnicodeBlock = UnicodeBlock {
    name: "Old Uyghur",
    start: 69488,
    end: 69551,
};
pub const CHORASMIAN: UnicodeBlock = UnicodeBlock {
    name: "Chorasmian",
    start: 69552,
    end: 69599,
};
pub const ELYMAIC: UnicodeBlock = UnicodeBlock {
    name: "Elymaic",
    start: 69600,
    end: 69631,
};
pub const BRAHMI: UnicodeBlock = UnicodeBlock {
    name: "Brahmi",
    start: 69632,
    end: 69759,
};
pub const KAITHI: UnicodeBlock = UnicodeBlock {
    name: "Kaithi",
    start: 69760,
    end: 69839,
};
pub const SORA_SOMPENG: UnicodeBlock = UnicodeBlock {
    name: "Sora Sompeng",
    start: 69840,
    end: 69887,
};
pub const CHAKMA: UnicodeBlock = UnicodeBlock {
    name: "Chakma",
    start: 69888,
    end: 69967,
};
pub const MAHAJANI: UnicodeBlock = UnicodeBlock {
    name: "Mahajani",
    start: 69968,
    end: 70015,
};
pub const SHARADA: UnicodeBlock = UnicodeBlock {
    name: "Sharada",
    start: 70016,
    end: 70111,
};
pub const SINHALA_ARCHAIC_NUMBERS: UnicodeBlock = UnicodeBlock {
    name: "Sinhala Archaic Numbers",
    start: 70112,
    end: 70143,
};
pub const KHOJKI: UnicodeBlock = UnicodeBlock {
    name: "Khojki",
    start: 70144,
    end: 70223,
};
pub const MULTANI: UnicodeBlock = UnicodeBlock {
    name: "Multani",
    start: 70272,
    end: 70319,
};
pub const KHUDAWADI: UnicodeBlock = UnicodeBlock {
    name: "Khudawadi",
    start: 70320,
    end: 70399,
};
pub const GRANTHA: UnicodeBlock = UnicodeBlock {
    name: "Grantha",
    start: 70400,
    end: 70527,
};
pub const NEWA: UnicodeBlock = UnicodeBlock {
    name: "Newa",
    start: 70656,
    end: 70783,
};
pub const TIRHUTA: UnicodeBlock = UnicodeBlock {
    name: "Tirhuta",
    start: 70784,
    end: 70879,
};
pub const SIDDHAM: UnicodeBlock = UnicodeBlock {
    name: "Siddham",
    start: 71040,
    end: 71167,
};
pub const MODI: UnicodeBlock = UnicodeBlock {
    name: "Modi",
    start: 71168,
    end: 71263,
};
pub const MONGOLIAN_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Mongolian Supplement",
    start: 71264,
    end: 71295,
};
pub const TAKRI: UnicodeBlock = UnicodeBlock {
    name: "Takri",
    start: 71296,
    end: 71375,
};
pub const AHOM: UnicodeBlock = UnicodeBlock {
    name: "Ahom",
    start: 71424,
    end: 71503,
};
pub const DOGRA: UnicodeBlock = UnicodeBlock {
    name: "Dogra",
    start: 71680,
    end: 71759,
};
pub const WARANG_CITI: UnicodeBlock = UnicodeBlock {
    name: "Warang Citi",
    start: 71840,
    end: 71935,
};
pub const DIVES_AKURU: UnicodeBlock = UnicodeBlock {
    name: "Dives Akuru",
    start: 71936,
    end: 72031,
};
pub const NANDINAGARI: UnicodeBlock = UnicodeBlock {
    name: "Nandinagari",
    start: 72096,
    end: 72191,
};
pub const ZANABAZAR_SQUARE: UnicodeBlock = UnicodeBlock {
    name: "Zanabazar Square",
    start: 72192,
    end: 72271,
};
pub const SOYOMBO: UnicodeBlock = UnicodeBlock {
    name: "Soyombo",
    start: 72272,
    end: 72367,
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Unified Canadian Aboriginal Syllabics Extended-A",
    start: 72368,
    end: 72383,
};
pub const PAU_CIN_HAU: UnicodeBlock = UnicodeBlock {
    name: "Pau Cin Hau",
    start: 72384,
    end: 72447,
};
pub const DEVANAGARI_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Devanagari Extended-A",
    start: 72448,
    end: 72543,
};
pub const BHAIKSUKI: UnicodeBlock = UnicodeBlock {
    name: "Bhaiksuki",
    start: 72704,
    end: 72815,
};
pub const MARCHEN: UnicodeBlock = UnicodeBlock {
    name: "Marchen",
    start: 72816,
    end: 72895,
};
pub const MASARAM_GONDI: UnicodeBlock = UnicodeBlock {
    name: "Masaram Gondi",
    start: 72960,
    end: 73055,
};
pub const GUNJALA_GONDI: UnicodeBlock = UnicodeBlock {
    name: "Gunjala Gondi",
    start: 73056,
    end: 73135,
};
pub const MAKASAR: UnicodeBlock = UnicodeBlock {
    name: "Makasar",
    start: 73440,
    end: 73471,
};
pub const KAWI: UnicodeBlock = UnicodeBlock {
    name: "Kawi",
    start: 73472,
    end: 73567,
};
pub const LISU_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Lisu Supplement",
    start: 73648,
    end: 73663,
};
pub const TAMIL_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Tamil Supplement",
    start: 73664,
    end: 73727,
};
pub const CUNEIFORM: UnicodeBlock = UnicodeBlock {
    name: "Cuneiform",
    start: 73728,
    end: 74751,
};
pub const CUNEIFORM_NUMBERS_AND_PUNCTUATION: UnicodeBlock = UnicodeBlock {
    name: "Cuneiform Numbers and Punctuation",
    start: 74752,
    end: 74879,
};
pub const EARLY_DYNASTIC_CUNEIFORM: UnicodeBlock = UnicodeBlock {
    name: "Early Dynastic Cuneiform",
    start: 74880,
    end: 75087,
};
pub const CYPRO_MINOAN: UnicodeBlock = UnicodeBlock {
    name: "Cypro-Minoan",
    start: 77712,
    end: 77823,
};
pub const EGYPTIAN_HIEROGLYPHS: UnicodeBlock = UnicodeBlock {
    name: "Egyptian Hieroglyphs",
    start: 77824,
    end: 78895,
};
pub const EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UnicodeBlock = UnicodeBlock {
    name: "Egyptian Hieroglyph Format Controls",
    start: 78896,
    end: 78943,
};
pub const ANATOLIAN_HIEROGLYPHS: UnicodeBlock = UnicodeBlock {
    name: "Anatolian Hieroglyphs",
    start: 82944,
    end: 83583,
};
pub const BAMUM_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Bamum Supplement",
    start: 92160,
    end: 92735,
};
pub const MRO: UnicodeBlock = UnicodeBlock {
    name: "Mro",
    start: 92736,
    end: 92783,
};
pub const TANGSA: UnicodeBlock = UnicodeBlock {
    name: "Tangsa",
    start: 92784,
    end: 92879,
};
pub const BASSA_VAH: UnicodeBlock = UnicodeBlock {
    name: "Bassa Vah",
    start: 92880,
    end: 92927,
};
pub const PAHAWH_HMONG: UnicodeBlock = UnicodeBlock {
    name: "Pahawh Hmong",
    start: 92928,
    end: 93071,
};
pub const MEDEFAIDRIN: UnicodeBlock = UnicodeBlock {
    name: "Medefaidrin",
    start: 93760,
    end: 93855,
};
pub const MIAO: UnicodeBlock = UnicodeBlock {
    name: "Miao",
    start: 93952,
    end: 94111,
};
pub const IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UnicodeBlock = UnicodeBlock {
    name: "Ideographic Symbols and Punctuation",
    start: 94176,
    end: 94207,
};
pub const TANGUT: UnicodeBlock = UnicodeBlock {
    name: "Tangut",
    start: 94208,
    end: 100351,
};
pub const TANGUT_COMPONENTS: UnicodeBlock = UnicodeBlock {
    name: "Tangut Components",
    start: 100352,
    end: 101119,
};
pub const KHITAN_SMALL_SCRIPT: UnicodeBlock = UnicodeBlock {
    name: "Khitan Small Script",
    start: 101120,
    end: 101631,
};
pub const TANGUT_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Tangut Supplement",
    start: 101632,
    end: 101759,
};
pub const KANA_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Kana Extended-B",
    start: 110576,
    end: 110591,
};
pub const KANA_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Kana Supplement",
    start: 110592,
    end: 110847,
};
pub const KANA_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Kana Extended-A",
    start: 110848,
    end: 110895,
};
pub const SMALL_KANA_EXTENSION: UnicodeBlock = UnicodeBlock {
    name: "Small Kana Extension",
    start: 110896,
    end: 110959,
};
pub const NUSHU: UnicodeBlock = UnicodeBlock {
    name: "Nushu",
    start: 110960,
    end: 111359,
};
pub const DUPLOYAN: UnicodeBlock = UnicodeBlock {
    name: "Duployan",
    start: 113664,
    end: 113823,
};
pub const SHORTHAND_FORMAT_CONTROLS: UnicodeBlock = UnicodeBlock {
    name: "Shorthand Format Controls",
    start: 113824,
    end: 113839,
};
pub const ZNAMENNY_MUSICAL_NOTATION: UnicodeBlock = UnicodeBlock {
    name: "Znamenny Musical Notation",
    start: 118528,
    end: 118735,
};
pub const BYZANTINE_MUSICAL_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Byzantine Musical Symbols",
    start: 118784,
    end: 119039,
};
pub const MUSICAL_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Musical Symbols",
    start: 119040,
    end: 119295,
};
pub const ANCIENT_GREEK_MUSICAL_NOTATION: UnicodeBlock = UnicodeBlock {
    name: "Ancient Greek Musical Notation",
    start: 119296,
    end: 119375,
};
pub const KAKTOVIK_NUMERALS: UnicodeBlock = UnicodeBlock {
    name: "Kaktovik Numerals",
    start: 119488,
    end: 119519,
};
pub const MAYAN_NUMERALS: UnicodeBlock = UnicodeBlock {
    name: "Mayan Numerals",
    start: 119520,
    end: 119551,
};
pub const TAI_XUAN_JING_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Tai Xuan Jing Symbols",
    start: 119552,
    end: 119647,
};
pub const COUNTING_ROD_NUMERALS: UnicodeBlock = UnicodeBlock {
    name: "Counting Rod Numerals",
    start: 119648,
    end: 119679,
};
pub const MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Mathematical Alphanumeric Symbols",
    start: 119808,
    end: 120831,
};
pub const SUTTON_SIGNWRITING: UnicodeBlock = UnicodeBlock {
    name: "Sutton SignWriting",
    start: 120832,
    end: 121519,
};
pub const LATIN_EXTENDED_G: UnicodeBlock = UnicodeBlock {
    name: "Latin Extended-G",
    start: 122624,
    end: 122879,
};
pub const GLAGOLITIC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Glagolitic Supplement",
    start: 122880,
    end: 122927,
};
pub const CYRILLIC_EXTENDED_D: UnicodeBlock = UnicodeBlock {
    name: "Cyrillic Extended-D",
    start: 122928,
    end: 123023,
};
pub const NYIAKENG_PUACHUE_HMONG: UnicodeBlock = UnicodeBlock {
    name: "Nyiakeng Puachue Hmong",
    start: 123136,
    end: 123215,
};
pub const TOTO: UnicodeBlock = UnicodeBlock {
    name: "Toto",
    start: 123536,
    end: 123583,
};
pub const WANCHO: UnicodeBlock = UnicodeBlock {
    name: "Wancho",
    start: 123584,
    end: 123647,
};
pub const NAG_MUNDARI: UnicodeBlock = UnicodeBlock {
    name: "Nag Mundari",
    start: 124112,
    end: 124159,
};
pub const ETHIOPIC_EXTENDED_B: UnicodeBlock = UnicodeBlock {
    name: "Ethiopic Extended-B",
    start: 124896,
    end: 124927,
};
pub const MENDE_KIKAKUI: UnicodeBlock = UnicodeBlock {
    name: "Mende Kikakui",
    start: 124928,
    end: 125151,
};
pub const ADLAM: UnicodeBlock = UnicodeBlock {
    name: "Adlam",
    start: 125184,
    end: 125279,
};
pub const INDIC_SIYAQ_NUMBERS: UnicodeBlock = UnicodeBlock {
    name: "Indic Siyaq Numbers",
    start: 126064,
    end: 126143,
};
pub const OTTOMAN_SIYAQ_NUMBERS: UnicodeBlock = UnicodeBlock {
    name: "Ottoman Siyaq Numbers",
    start: 126208,
    end: 126287,
};
pub const ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Arabic Mathematical Alphabetic Symbols",
    start: 126464,
    end: 126719,
};
pub const MAHJONG_TILES: UnicodeBlock = UnicodeBlock {
    name: "Mahjong Tiles",
    start: 126976,
    end: 127023,
};
pub const DOMINO_TILES: UnicodeBlock = UnicodeBlock {
    name: "Domino Tiles",
    start: 127024,
    end: 127135,
};
pub const PLAYING_CARDS: UnicodeBlock = UnicodeBlock {
    name: "Playing Cards",
    start: 127136,
    end: 127231,
};
pub const ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Enclosed Alphanumeric Supplement",
    start: 127232,
    end: 127487,
};
pub const ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Enclosed Ideographic Supplement",
    start: 127488,
    end: 127743,
};
pub const MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UnicodeBlock = UnicodeBlock {
    name: "Miscellaneous Symbols and Pictographs",
    start: 127744,
    end: 128511,
};
pub const EMOTICONS: UnicodeBlock = UnicodeBlock {
    name: "Emoticons",
    start: 128512,
    end: 128591,
};
pub const ORNAMENTAL_DINGBATS: UnicodeBlock = UnicodeBlock {
    name: "Ornamental Dingbats",
    start: 128592,
    end: 128639,
};
pub const TRANSPORT_AND_MAP_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Transport and Map Symbols",
    start: 128640,
    end: 128767,
};
pub const ALCHEMICAL_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Alchemical Symbols",
    start: 128768,
    end: 128895,
};
pub const GEOMETRIC_SHAPES_EXTENDED: UnicodeBlock = UnicodeBlock {
    name: "Geometric Shapes Extended",
    start: 128896,
    end: 129023,
};
pub const SUPPLEMENTAL_ARROWS_C: UnicodeBlock = UnicodeBlock {
    name: "Supplemental Arrows-C",
    start: 129024,
    end: 129279,
};
pub const SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UnicodeBlock = UnicodeBlock {
    name: "Supplemental Symbols and Pictographs",
    start: 129280,
    end: 129535,
};
pub const CHESS_SYMBOLS: UnicodeBlock = UnicodeBlock {
    name: "Chess Symbols",
    start: 129536,
    end: 129647,
};
pub const SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UnicodeBlock = UnicodeBlock {
    name: "Symbols and Pictographs Extended-A",
    start: 129648,
    end: 129791,
};
pub const SYMBOLS_FOR_LEGACY_COMPUTING: UnicodeBlock = UnicodeBlock {
    name: "Symbols for Legacy Computing",
    start: 129792,
    end: 130047,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension B",
    start: 131072,
    end: 173791,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension C",
    start: 173824,
    end: 177983,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension D",
    start: 177984,
    end: 178207,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension E",
    start: 178208,
    end: 183983,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension F",
    start: 183984,
    end: 191471,
};
pub const CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "CJK Compatibility Ideographs Supplement",
    start: 194560,
    end: 195103,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension G",
    start: 196608,
    end: 201551,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H: UnicodeBlock = UnicodeBlock {
    name: "CJK Unified Ideographs Extension H",
    start: 201552,
    end: 205743,
};
pub const TAGS: UnicodeBlock = UnicodeBlock {
    name: "Tags",
    start: 917504,
    end: 917631,
};
pub const VARIATION_SELECTORS_SUPPLEMENT: UnicodeBlock = UnicodeBlock {
    name: "Variation Selectors Supplement",
    start: 917760,
    end: 917999,
};
pub const SUPPLEMENTARY_PRIVATE_USE_AREA_A: UnicodeBlock = UnicodeBlock {
    name: "Supplementary Private Use Area-A",
    start: 983040,
    end: 1048575,
};
pub const SUPPLEMENTARY_PRIVATE_USE_AREA_B: UnicodeBlock = UnicodeBlock {
    name: "Supplementary Private Use Area-B",
    start: 1048576,
    end: 1114111,
};

/// Given a character, determine what unicode block contains it.
pub fn find_unicode_block(c: char) -> Option<UnicodeBlock> {
    match c {
        '\u{000000}'..='\u{00007F}' => Some(BASIC_LATIN),
        '\u{000080}'..='\u{0000FF}' => Some(LATIN_1_SUPPLEMENT),
        '\u{000100}'..='\u{00017F}' => Some(LATIN_EXTENDED_A),
        '\u{000180}'..='\u{00024F}' => Some(LATIN_EXTENDED_B),
        '\u{000250}'..='\u{0002AF}' => Some(IPA_EXTENSIONS),
        '\u{0002B0}'..='\u{0002FF}' => Some(SPACING_MODIFIER_LETTERS),
        '\u{000300}'..='\u{00036F}' => Some(COMBINING_DIACRITICAL_MARKS),
        '\u{000370}'..='\u{0003FF}' => Some(GREEK_AND_COPTIC),
        '\u{000400}'..='\u{0004FF}' => Some(CYRILLIC),
        '\u{000500}'..='\u{00052F}' => Some(CYRILLIC_SUPPLEMENT),
        '\u{000530}'..='\u{00058F}' => Some(ARMENIAN),
        '\u{000590}'..='\u{0005FF}' => Some(HEBREW),
        '\u{000600}'..='\u{0006FF}' => Some(ARABIC),
        '\u{000700}'..='\u{00074F}' => Some(SYRIAC),
        '\u{000750}'..='\u{00077F}' => Some(ARABIC_SUPPLEMENT),
        '\u{000780}'..='\u{0007BF}' => Some(THAANA),
        '\u{0007C0}'..='\u{0007FF}' => Some(NKO),
        '\u{000800}'..='\u{00083F}' => Some(SAMARITAN),
        '\u{000840}'..='\u{00085F}' => Some(MANDAIC),
        '\u{000860}'..='\u{00086F}' => Some(SYRIAC_SUPPLEMENT),
        '\u{000870}'..='\u{00089F}' => Some(ARABIC_EXTENDED_B),
        '\u{0008A0}'..='\u{0008FF}' => Some(ARABIC_EXTENDED_A),
        '\u{000900}'..='\u{00097F}' => Some(DEVANAGARI),
        '\u{000980}'..='\u{0009FF}' => Some(BENGALI),
        '\u{000A00}'..='\u{000A7F}' => Some(GURMUKHI),
        '\u{000A80}'..='\u{000AFF}' => Some(GUJARATI),
        '\u{000B00}'..='\u{000B7F}' => Some(ORIYA),
        '\u{000B80}'..='\u{000BFF}' => Some(TAMIL),
        '\u{000C00}'..='\u{000C7F}' => Some(TELUGU),
        '\u{000C80}'..='\u{000CFF}' => Some(KANNADA),
        '\u{000D00}'..='\u{000D7F}' => Some(MALAYALAM),
        '\u{000D80}'..='\u{000DFF}' => Some(SINHALA),
        '\u{000E00}'..='\u{000E7F}' => Some(THAI),
        '\u{000E80}'..='\u{000EFF}' => Some(LAO),
        '\u{000F00}'..='\u{000FFF}' => Some(TIBETAN),
        '\u{001000}'..='\u{00109F}' => Some(MYANMAR),
        '\u{0010A0}'..='\u{0010FF}' => Some(GEORGIAN),
        '\u{001100}'..='\u{0011FF}' => Some(HANGUL_JAMO),
        '\u{001200}'..='\u{00137F}' => Some(ETHIOPIC),
        '\u{001380}'..='\u{00139F}' => Some(ETHIOPIC_SUPPLEMENT),
        '\u{0013A0}'..='\u{0013FF}' => Some(CHEROKEE),
        '\u{001400}'..='\u{00167F}' => Some(UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS),
        '\u{001680}'..='\u{00169F}' => Some(OGHAM),
        '\u{0016A0}'..='\u{0016FF}' => Some(RUNIC),
        '\u{001700}'..='\u{00171F}' => Some(TAGALOG),
        '\u{001720}'..='\u{00173F}' => Some(HANUNOO),
        '\u{001740}'..='\u{00175F}' => Some(BUHID),
        '\u{001760}'..='\u{00177F}' => Some(TAGBANWA),
        '\u{001780}'..='\u{0017FF}' => Some(KHMER),
        '\u{001800}'..='\u{0018AF}' => Some(MONGOLIAN),
        '\u{0018B0}'..='\u{0018FF}' => Some(UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED),
        '\u{001900}'..='\u{00194F}' => Some(LIMBU),
        '\u{001950}'..='\u{00197F}' => Some(TAI_LE),
        '\u{001980}'..='\u{0019DF}' => Some(NEW_TAI_LUE),
        '\u{0019E0}'..='\u{0019FF}' => Some(KHMER_SYMBOLS),
        '\u{001A00}'..='\u{001A1F}' => Some(BUGINESE),
        '\u{001A20}'..='\u{001AAF}' => Some(TAI_THAM),
        '\u{001AB0}'..='\u{001AFF}' => Some(COMBINING_DIACRITICAL_MARKS_EXTENDED),
        '\u{001B00}'..='\u{001B7F}' => Some(BALINESE),
        '\u{001B80}'..='\u{001BBF}' => Some(SUNDANESE),
        '\u{001BC0}'..='\u{001BFF}' => Some(BATAK),
        '\u{001C00}'..='\u{001C4F}' => Some(LEPCHA),
        '\u{001C50}'..='\u{001C7F}' => Some(OL_CHIKI),
        '\u{001C80}'..='\u{001C8F}' => Some(CYRILLIC_EXTENDED_C),
        '\u{001C90}'..='\u{001CBF}' => Some(GEORGIAN_EXTENDED),
        '\u{001CC0}'..='\u{001CCF}' => Some(SUNDANESE_SUPPLEMENT),
        '\u{001CD0}'..='\u{001CFF}' => Some(VEDIC_EXTENSIONS),
        '\u{001D00}'..='\u{001D7F}' => Some(PHONETIC_EXTENSIONS),
        '\u{001D80}'..='\u{001DBF}' => Some(PHONETIC_EXTENSIONS_SUPPLEMENT),
        '\u{001DC0}'..='\u{001DFF}' => Some(COMBINING_DIACRITICAL_MARKS_SUPPLEMENT),
        '\u{001E00}'..='\u{001EFF}' => Some(LATIN_EXTENDED_ADDITIONAL),
        '\u{001F00}'..='\u{001FFF}' => Some(GREEK_EXTENDED),
        '\u{002000}'..='\u{00206F}' => Some(GENERAL_PUNCTUATION),
        '\u{002070}'..='\u{00209F}' => Some(SUPERSCRIPTS_AND_SUBSCRIPTS),
        '\u{0020A0}'..='\u{0020CF}' => Some(CURRENCY_SYMBOLS),
        '\u{0020D0}'..='\u{0020FF}' => Some(COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS),
        '\u{002100}'..='\u{00214F}' => Some(LETTERLIKE_SYMBOLS),
        '\u{002150}'..='\u{00218F}' => Some(NUMBER_FORMS),
        '\u{002190}'..='\u{0021FF}' => Some(ARROWS),
        '\u{002200}'..='\u{0022FF}' => Some(MATHEMATICAL_OPERATORS),
        '\u{002300}'..='\u{0023FF}' => Some(MISCELLANEOUS_TECHNICAL),
        '\u{002400}'..='\u{00243F}' => Some(CONTROL_PICTURES),
        '\u{002440}'..='\u{00245F}' => Some(OPTICAL_CHARACTER_RECOGNITION),
        '\u{002460}'..='\u{0024FF}' => Some(ENCLOSED_ALPHANUMERICS),
        '\u{002500}'..='\u{00257F}' => Some(BOX_DRAWING),
        '\u{002580}'..='\u{00259F}' => Some(BLOCK_ELEMENTS),
        '\u{0025A0}'..='\u{0025FF}' => Some(GEOMETRIC_SHAPES),
        '\u{002600}'..='\u{0026FF}' => Some(MISCELLANEOUS_SYMBOLS),
        '\u{002700}'..='\u{0027BF}' => Some(DINGBATS),
        '\u{0027C0}'..='\u{0027EF}' => Some(MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A),
        '\u{0027F0}'..='\u{0027FF}' => Some(SUPPLEMENTAL_ARROWS_A),
        '\u{002800}'..='\u{0028FF}' => Some(BRAILLE_PATTERNS),
        '\u{002900}'..='\u{00297F}' => Some(SUPPLEMENTAL_ARROWS_B),
        '\u{002980}'..='\u{0029FF}' => Some(MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B),
        '\u{002A00}'..='\u{002AFF}' => Some(SUPPLEMENTAL_MATHEMATICAL_OPERATORS),
        '\u{002B00}'..='\u{002BFF}' => Some(MISCELLANEOUS_SYMBOLS_AND_ARROWS),
        '\u{002C00}'..='\u{002C5F}' => Some(GLAGOLITIC),
        '\u{002C60}'..='\u{002C7F}' => Some(LATIN_EXTENDED_C),
        '\u{002C80}'..='\u{002CFF}' => Some(COPTIC),
        '\u{002D00}'..='\u{002D2F}' => Some(GEORGIAN_SUPPLEMENT),
        '\u{002D30}'..='\u{002D7F}' => Some(TIFINAGH),
        '\u{002D80}'..='\u{002DDF}' => Some(ETHIOPIC_EXTENDED),
        '\u{002DE0}'..='\u{002DFF}' => Some(CYRILLIC_EXTENDED_A),
        '\u{002E00}'..='\u{002E7F}' => Some(SUPPLEMENTAL_PUNCTUATION),
        '\u{002E80}'..='\u{002EFF}' => Some(CJK_RADICALS_SUPPLEMENT),
        '\u{002F00}'..='\u{002FDF}' => Some(KANGXI_RADICALS),
        '\u{002FF0}'..='\u{002FFF}' => Some(IDEOGRAPHIC_DESCRIPTION_CHARACTERS),
        '\u{003000}'..='\u{00303F}' => Some(CJK_SYMBOLS_AND_PUNCTUATION),
        '\u{003040}'..='\u{00309F}' => Some(HIRAGANA),
        '\u{0030A0}'..='\u{0030FF}' => Some(KATAKANA),
        '\u{003100}'..='\u{00312F}' => Some(BOPOMOFO),
        '\u{003130}'..='\u{00318F}' => Some(HANGUL_COMPATIBILITY_JAMO),
        '\u{003190}'..='\u{00319F}' => Some(KANBUN),
        '\u{0031A0}'..='\u{0031BF}' => Some(BOPOMOFO_EXTENDED),
        '\u{0031C0}'..='\u{0031EF}' => Some(CJK_STROKES),
        '\u{0031F0}'..='\u{0031FF}' => Some(KATAKANA_PHONETIC_EXTENSIONS),
        '\u{003200}'..='\u{0032FF}' => Some(ENCLOSED_CJK_LETTERS_AND_MONTHS),
        '\u{003300}'..='\u{0033FF}' => Some(CJK_COMPATIBILITY),
        '\u{003400}'..='\u{004DBF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A),
        '\u{004DC0}'..='\u{004DFF}' => Some(YIJING_HEXAGRAM_SYMBOLS),
        '\u{004E00}'..='\u{009FFF}' => Some(CJK_UNIFIED_IDEOGRAPHS),
        '\u{00A000}'..='\u{00A48F}' => Some(YI_SYLLABLES),
        '\u{00A490}'..='\u{00A4CF}' => Some(YI_RADICALS),
        '\u{00A4D0}'..='\u{00A4FF}' => Some(LISU),
        '\u{00A500}'..='\u{00A63F}' => Some(VAI),
        '\u{00A640}'..='\u{00A69F}' => Some(CYRILLIC_EXTENDED_B),
        '\u{00A6A0}'..='\u{00A6FF}' => Some(BAMUM),
        '\u{00A700}'..='\u{00A71F}' => Some(MODIFIER_TONE_LETTERS),
        '\u{00A720}'..='\u{00A7FF}' => Some(LATIN_EXTENDED_D),
        '\u{00A800}'..='\u{00A82F}' => Some(SYLOTI_NAGRI),
        '\u{00A830}'..='\u{00A83F}' => Some(COMMON_INDIC_NUMBER_FORMS),
        '\u{00A840}'..='\u{00A87F}' => Some(PHAGS_PA),
        '\u{00A880}'..='\u{00A8DF}' => Some(SAURASHTRA),
        '\u{00A8E0}'..='\u{00A8FF}' => Some(DEVANAGARI_EXTENDED),
        '\u{00A900}'..='\u{00A92F}' => Some(KAYAH_LI),
        '\u{00A930}'..='\u{00A95F}' => Some(REJANG),
        '\u{00A960}'..='\u{00A97F}' => Some(HANGUL_JAMO_EXTENDED_A),
        '\u{00A980}'..='\u{00A9DF}' => Some(JAVANESE),
        '\u{00A9E0}'..='\u{00A9FF}' => Some(MYANMAR_EXTENDED_B),
        '\u{00AA00}'..='\u{00AA5F}' => Some(CHAM),
        '\u{00AA60}'..='\u{00AA7F}' => Some(MYANMAR_EXTENDED_A),
        '\u{00AA80}'..='\u{00AADF}' => Some(TAI_VIET),
        '\u{00AAE0}'..='\u{00AAFF}' => Some(MEETEI_MAYEK_EXTENSIONS),
        '\u{00AB00}'..='\u{00AB2F}' => Some(ETHIOPIC_EXTENDED_A),
        '\u{00AB30}'..='\u{00AB6F}' => Some(LATIN_EXTENDED_E),
        '\u{00AB70}'..='\u{00ABBF}' => Some(CHEROKEE_SUPPLEMENT),
        '\u{00ABC0}'..='\u{00ABFF}' => Some(MEETEI_MAYEK),
        '\u{00AC00}'..='\u{00D7AF}' => Some(HANGUL_SYLLABLES),
        '\u{00D7B0}'..='\u{00D7FF}' => Some(HANGUL_JAMO_EXTENDED_B),
        '\u{00E000}'..='\u{00F8FF}' => Some(PRIVATE_USE_AREA),
        '\u{00F900}'..='\u{00FAFF}' => Some(CJK_COMPATIBILITY_IDEOGRAPHS),
        '\u{00FB00}'..='\u{00FB4F}' => Some(ALPHABETIC_PRESENTATION_FORMS),
        '\u{00FB50}'..='\u{00FDFF}' => Some(ARABIC_PRESENTATION_FORMS_A),
        '\u{00FE00}'..='\u{00FE0F}' => Some(VARIATION_SELECTORS),
        '\u{00FE10}'..='\u{00FE1F}' => Some(VERTICAL_FORMS),
        '\u{00FE20}'..='\u{00FE2F}' => Some(COMBINING_HALF_MARKS),
        '\u{00FE30}'..='\u{00FE4F}' => Some(CJK_COMPATIBILITY_FORMS),
        '\u{00FE50}'..='\u{00FE6F}' => Some(SMALL_FORM_VARIANTS),
        '\u{00FE70}'..='\u{00FEFF}' => Some(ARABIC_PRESENTATION_FORMS_B),
        '\u{00FF00}'..='\u{00FFEF}' => Some(HALFWIDTH_AND_FULLWIDTH_FORMS),
        '\u{00FFF0}'..='\u{00FFFF}' => Some(SPECIALS),
        '\u{010000}'..='\u{01007F}' => Some(LINEAR_B_SYLLABARY),
        '\u{010080}'..='\u{0100FF}' => Some(LINEAR_B_IDEOGRAMS),
        '\u{010100}'..='\u{01013F}' => Some(AEGEAN_NUMBERS),
        '\u{010140}'..='\u{01018F}' => Some(ANCIENT_GREEK_NUMBERS),
        '\u{010190}'..='\u{0101CF}' => Some(ANCIENT_SYMBOLS),
        '\u{0101D0}'..='\u{0101FF}' => Some(PHAISTOS_DISC),
        '\u{010280}'..='\u{01029F}' => Some(LYCIAN),
        '\u{0102A0}'..='\u{0102DF}' => Some(CARIAN),
        '\u{0102E0}'..='\u{0102FF}' => Some(COPTIC_EPACT_NUMBERS),
        '\u{010300}'..='\u{01032F}' => Some(OLD_ITALIC),
        '\u{010330}'..='\u{01034F}' => Some(GOTHIC),
        '\u{010350}'..='\u{01037F}' => Some(OLD_PERMIC),
        '\u{010380}'..='\u{01039F}' => Some(UGARITIC),
        '\u{0103A0}'..='\u{0103DF}' => Some(OLD_PERSIAN),
        '\u{010400}'..='\u{01044F}' => Some(DESERET),
        '\u{010450}'..='\u{01047F}' => Some(SHAVIAN),
        '\u{010480}'..='\u{0104AF}' => Some(OSMANYA),
        '\u{0104B0}'..='\u{0104FF}' => Some(OSAGE),
        '\u{010500}'..='\u{01052F}' => Some(ELBASAN),
        '\u{010530}'..='\u{01056F}' => Some(CAUCASIAN_ALBANIAN),
        '\u{010570}'..='\u{0105BF}' => Some(VITHKUQI),
        '\u{010600}'..='\u{01077F}' => Some(LINEAR_A),
        '\u{010780}'..='\u{0107BF}' => Some(LATIN_EXTENDED_F),
        '\u{010800}'..='\u{01083F}' => Some(CYPRIOT_SYLLABARY),
        '\u{010840}'..='\u{01085F}' => Some(IMPERIAL_ARAMAIC),
        '\u{010860}'..='\u{01087F}' => Some(PALMYRENE),
        '\u{010880}'..='\u{0108AF}' => Some(NABATAEAN),
        '\u{0108E0}'..='\u{0108FF}' => Some(HATRAN),
        '\u{010900}'..='\u{01091F}' => Some(PHOENICIAN),
        '\u{010920}'..='\u{01093F}' => Some(LYDIAN),
        '\u{010980}'..='\u{01099F}' => Some(MEROITIC_HIEROGLYPHS),
        '\u{0109A0}'..='\u{0109FF}' => Some(MEROITIC_CURSIVE),
        '\u{010A00}'..='\u{010A5F}' => Some(KHAROSHTHI),
        '\u{010A60}'..='\u{010A7F}' => Some(OLD_SOUTH_ARABIAN),
        '\u{010A80}'..='\u{010A9F}' => Some(OLD_NORTH_ARABIAN),
        '\u{010AC0}'..='\u{010AFF}' => Some(MANICHAEAN),
        '\u{010B00}'..='\u{010B3F}' => Some(AVESTAN),
        '\u{010B40}'..='\u{010B5F}' => Some(INSCRIPTIONAL_PARTHIAN),
        '\u{010B60}'..='\u{010B7F}' => Some(INSCRIPTIONAL_PAHLAVI),
        '\u{010B80}'..='\u{010BAF}' => Some(PSALTER_PAHLAVI),
        '\u{010C00}'..='\u{010C4F}' => Some(OLD_TURKIC),
        '\u{010C80}'..='\u{010CFF}' => Some(OLD_HUNGARIAN),
        '\u{010D00}'..='\u{010D3F}' => Some(HANIFI_ROHINGYA),
        '\u{010E60}'..='\u{010E7F}' => Some(RUMI_NUMERAL_SYMBOLS),
        '\u{010E80}'..='\u{010EBF}' => Some(YEZIDI),
        '\u{010EC0}'..='\u{010EFF}' => Some(ARABIC_EXTENDED_C),
        '\u{010F00}'..='\u{010F2F}' => Some(OLD_SOGDIAN),
        '\u{010F30}'..='\u{010F6F}' => Some(SOGDIAN),
        '\u{010F70}'..='\u{010FAF}' => Some(OLD_UYGHUR),
        '\u{010FB0}'..='\u{010FDF}' => Some(CHORASMIAN),
        '\u{010FE0}'..='\u{010FFF}' => Some(ELYMAIC),
        '\u{011000}'..='\u{01107F}' => Some(BRAHMI),
        '\u{011080}'..='\u{0110CF}' => Some(KAITHI),
        '\u{0110D0}'..='\u{0110FF}' => Some(SORA_SOMPENG),
        '\u{011100}'..='\u{01114F}' => Some(CHAKMA),
        '\u{011150}'..='\u{01117F}' => Some(MAHAJANI),
        '\u{011180}'..='\u{0111DF}' => Some(SHARADA),
        '\u{0111E0}'..='\u{0111FF}' => Some(SINHALA_ARCHAIC_NUMBERS),
        '\u{011200}'..='\u{01124F}' => Some(KHOJKI),
        '\u{011280}'..='\u{0112AF}' => Some(MULTANI),
        '\u{0112B0}'..='\u{0112FF}' => Some(KHUDAWADI),
        '\u{011300}'..='\u{01137F}' => Some(GRANTHA),
        '\u{011400}'..='\u{01147F}' => Some(NEWA),
        '\u{011480}'..='\u{0114DF}' => Some(TIRHUTA),
        '\u{011580}'..='\u{0115FF}' => Some(SIDDHAM),
        '\u{011600}'..='\u{01165F}' => Some(MODI),
        '\u{011660}'..='\u{01167F}' => Some(MONGOLIAN_SUPPLEMENT),
        '\u{011680}'..='\u{0116CF}' => Some(TAKRI),
        '\u{011700}'..='\u{01174F}' => Some(AHOM),
        '\u{011800}'..='\u{01184F}' => Some(DOGRA),
        '\u{0118A0}'..='\u{0118FF}' => Some(WARANG_CITI),
        '\u{011900}'..='\u{01195F}' => Some(DIVES_AKURU),
        '\u{0119A0}'..='\u{0119FF}' => Some(NANDINAGARI),
        '\u{011A00}'..='\u{011A4F}' => Some(ZANABAZAR_SQUARE),
        '\u{011A50}'..='\u{011AAF}' => Some(SOYOMBO),
        '\u{011AB0}'..='\u{011ABF}' => Some(UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED_A),
        '\u{011AC0}'..='\u{011AFF}' => Some(PAU_CIN_HAU),
        '\u{011B00}'..='\u{011B5F}' => Some(DEVANAGARI_EXTENDED_A),
        '\u{011C00}'..='\u{011C6F}' => Some(BHAIKSUKI),
        '\u{011C70}'..='\u{011CBF}' => Some(MARCHEN),
        '\u{011D00}'..='\u{011D5F}' => Some(MASARAM_GONDI),
        '\u{011D60}'..='\u{011DAF}' => Some(GUNJALA_GONDI),
        '\u{011EE0}'..='\u{011EFF}' => Some(MAKASAR),
        '\u{011F00}'..='\u{011F5F}' => Some(KAWI),
        '\u{011FB0}'..='\u{011FBF}' => Some(LISU_SUPPLEMENT),
        '\u{011FC0}'..='\u{011FFF}' => Some(TAMIL_SUPPLEMENT),
        '\u{012000}'..='\u{0123FF}' => Some(CUNEIFORM),
        '\u{012400}'..='\u{01247F}' => Some(CUNEIFORM_NUMBERS_AND_PUNCTUATION),
        '\u{012480}'..='\u{01254F}' => Some(EARLY_DYNASTIC_CUNEIFORM),
        '\u{012F90}'..='\u{012FFF}' => Some(CYPRO_MINOAN),
        '\u{013000}'..='\u{01342F}' => Some(EGYPTIAN_HIEROGLYPHS),
        '\u{013430}'..='\u{01345F}' => Some(EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS),
        '\u{014400}'..='\u{01467F}' => Some(ANATOLIAN_HIEROGLYPHS),
        '\u{016800}'..='\u{016A3F}' => Some(BAMUM_SUPPLEMENT),
        '\u{016A40}'..='\u{016A6F}' => Some(MRO),
        '\u{016A70}'..='\u{016ACF}' => Some(TANGSA),
        '\u{016AD0}'..='\u{016AFF}' => Some(BASSA_VAH),
        '\u{016B00}'..='\u{016B8F}' => Some(PAHAWH_HMONG),
        '\u{016E40}'..='\u{016E9F}' => Some(MEDEFAIDRIN),
        '\u{016F00}'..='\u{016F9F}' => Some(MIAO),
        '\u{016FE0}'..='\u{016FFF}' => Some(IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION),
        '\u{017000}'..='\u{0187FF}' => Some(TANGUT),
        '\u{018800}'..='\u{018AFF}' => Some(TANGUT_COMPONENTS),
        '\u{018B00}'..='\u{018CFF}' => Some(KHITAN_SMALL_SCRIPT),
        '\u{018D00}'..='\u{018D7F}' => Some(TANGUT_SUPPLEMENT),
        '\u{01AFF0}'..='\u{01AFFF}' => Some(KANA_EXTENDED_B),
        '\u{01B000}'..='\u{01B0FF}' => Some(KANA_SUPPLEMENT),
        '\u{01B100}'..='\u{01B12F}' => Some(KANA_EXTENDED_A),
        '\u{01B130}'..='\u{01B16F}' => Some(SMALL_KANA_EXTENSION),
        '\u{01B170}'..='\u{01B2FF}' => Some(NUSHU),
        '\u{01BC00}'..='\u{01BC9F}' => Some(DUPLOYAN),
        '\u{01BCA0}'..='\u{01BCAF}' => Some(SHORTHAND_FORMAT_CONTROLS),
        '\u{01CF00}'..='\u{01CFCF}' => Some(ZNAMENNY_MUSICAL_NOTATION),
        '\u{01D000}'..='\u{01D0FF}' => Some(BYZANTINE_MUSICAL_SYMBOLS),
        '\u{01D100}'..='\u{01D1FF}' => Some(MUSICAL_SYMBOLS),
        '\u{01D200}'..='\u{01D24F}' => Some(ANCIENT_GREEK_MUSICAL_NOTATION),
        '\u{01D2C0}'..='\u{01D2DF}' => Some(KAKTOVIK_NUMERALS),
        '\u{01D2E0}'..='\u{01D2FF}' => Some(MAYAN_NUMERALS),
        '\u{01D300}'..='\u{01D35F}' => Some(TAI_XUAN_JING_SYMBOLS),
        '\u{01D360}'..='\u{01D37F}' => Some(COUNTING_ROD_NUMERALS),
        '\u{01D400}'..='\u{01D7FF}' => Some(MATHEMATICAL_ALPHANUMERIC_SYMBOLS),
        '\u{01D800}'..='\u{01DAAF}' => Some(SUTTON_SIGNWRITING),
        '\u{01DF00}'..='\u{01DFFF}' => Some(LATIN_EXTENDED_G),
        '\u{01E000}'..='\u{01E02F}' => Some(GLAGOLITIC_SUPPLEMENT),
        '\u{01E030}'..='\u{01E08F}' => Some(CYRILLIC_EXTENDED_D),
        '\u{01E100}'..='\u{01E14F}' => Some(NYIAKENG_PUACHUE_HMONG),
        '\u{01E290}'..='\u{01E2BF}' => Some(TOTO),
        '\u{01E2C0}'..='\u{01E2FF}' => Some(WANCHO),
        '\u{01E4D0}'..='\u{01E4FF}' => Some(NAG_MUNDARI),
        '\u{01E7E0}'..='\u{01E7FF}' => Some(ETHIOPIC_EXTENDED_B),
        '\u{01E800}'..='\u{01E8DF}' => Some(MENDE_KIKAKUI),
        '\u{01E900}'..='\u{01E95F}' => Some(ADLAM),
        '\u{01EC70}'..='\u{01ECBF}' => Some(INDIC_SIYAQ_NUMBERS),
        '\u{01ED00}'..='\u{01ED4F}' => Some(OTTOMAN_SIYAQ_NUMBERS),
        '\u{01EE00}'..='\u{01EEFF}' => Some(ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS),
        '\u{01F000}'..='\u{01F02F}' => Some(MAHJONG_TILES),
        '\u{01F030}'..='\u{01F09F}' => Some(DOMINO_TILES),
        '\u{01F0A0}'..='\u{01F0FF}' => Some(PLAYING_CARDS),
        '\u{01F100}'..='\u{01F1FF}' => Some(ENCLOSED_ALPHANUMERIC_SUPPLEMENT),
        '\u{01F200}'..='\u{01F2FF}' => Some(ENCLOSED_IDEOGRAPHIC_SUPPLEMENT),
        '\u{01F300}'..='\u{01F5FF}' => Some(MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS),
        '\u{01F600}'..='\u{01F64F}' => Some(EMOTICONS),
        '\u{01F650}'..='\u{01F67F}' => Some(ORNAMENTAL_DINGBATS),
        '\u{01F680}'..='\u{01F6FF}' => Some(TRANSPORT_AND_MAP_SYMBOLS),
        '\u{01F700}'..='\u{01F77F}' => Some(ALCHEMICAL_SYMBOLS),
        '\u{01F780}'..='\u{01F7FF}' => Some(GEOMETRIC_SHAPES_EXTENDED),
        '\u{01F800}'..='\u{01F8FF}' => Some(SUPPLEMENTAL_ARROWS_C),
        '\u{01F900}'..='\u{01F9FF}' => Some(SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS),
        '\u{01FA00}'..='\u{01FA6F}' => Some(CHESS_SYMBOLS),
        '\u{01FA70}'..='\u{01FAFF}' => Some(SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A),
        '\u{01FB00}'..='\u{01FBFF}' => Some(SYMBOLS_FOR_LEGACY_COMPUTING),
        '\u{020000}'..='\u{02A6DF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B),
        '\u{02A700}'..='\u{02B73F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C),
        '\u{02B740}'..='\u{02B81F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D),
        '\u{02B820}'..='\u{02CEAF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E),
        '\u{02CEB0}'..='\u{02EBEF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F),
        '\u{02F800}'..='\u{02FA1F}' => Some(CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT),
        '\u{030000}'..='\u{03134F}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G),
        '\u{031350}'..='\u{0323AF}' => Some(CJK_UNIFIED_IDEOGRAPHS_EXTENSION_H),
        '\u{0E0000}'..='\u{0E007F}' => Some(TAGS),
        '\u{0E0100}'..='\u{0E01EF}' => Some(VARIATION_SELECTORS_SUPPLEMENT),
        '\u{0F0000}'..='\u{0FFFFF}' => Some(SUPPLEMENTARY_PRIVATE_USE_AREA_A),
        '\u{100000}'..='\u{10FFFF}' => Some(SUPPLEMENTARY_PRIVATE_USE_AREA_B),
        _ => None,
    }
}
