#[test]
fn find_block() {
    assert_eq!(
        unicode_blocks::CJK_UNIFIED_IDEOGRAPHS,
        unicode_blocks::find_unicode_block('中').unwrap()
    );
    assert_eq!(
        unicode_blocks::CJK_SYMBOLS_AND_PUNCTUATION,
        unicode_blocks::find_unicode_block('。').unwrap()
    );
    assert_eq!(
        unicode_blocks::HALFWIDTH_AND_FULLWIDTH_FORMS,
        unicode_blocks::find_unicode_block('，').unwrap()
    );
}

#[test]
fn is_cjk() {
    assert!(!unicode_blocks::is_cjk('1'));
    assert!(!unicode_blocks::is_cjk('a'));
    assert!(!unicode_blocks::is_cjk('â'));
    assert!(!unicode_blocks::is_cjk('/'));
    assert!(!unicode_blocks::is_cjk('ß'));
    assert!(unicode_blocks::is_cjk('中'));
    assert!(unicode_blocks::is_cjk('𩸽'));
    assert!(unicode_blocks::is_cjk('。'));
    assert!(unicode_blocks::is_cjk('，'));
    assert!(unicode_blocks::is_cjk('あ'));
    assert!(unicode_blocks::is_cjk('ア'));
    assert!(unicode_blocks::is_cjk('を'));
    assert!(unicode_blocks::is_cjk('ヲ'));
    assert!(unicode_blocks::is_cjk('ん'));
    assert!(unicode_blocks::is_cjk('ン'));
    assert!(unicode_blocks::is_cjk('이'));
}

#[test]
fn is_cjk_block() {
    assert!(!unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('1').unwrap()));
    assert!(!unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('a').unwrap()));
    assert!(!unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('/').unwrap()));
    assert!(!unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('ß').unwrap()));
    assert!(unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('中').unwrap()));
    assert!(unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('。').unwrap()));
    assert!(unicode_blocks::is_cjk_block(unicode_blocks::find_unicode_block('，').unwrap()));
}
