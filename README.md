Unicode Blocks
====================

[![CI](https://github.com/magiclen/unicode-blocks/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/unicode-blocks/actions/workflows/ci.yml)

This crate contains a list of all unicode blocks and provides some functions to search across them.

Unicode version used by this crate: 18.0.0

## Examples

#### Find the Unicode block that contains a character.

```rust
assert_eq!(Some(unicode_blocks::BASIC_LATIN), unicode_blocks::find_unicode_block('A'));
```

## CJK blocks

The CJK helpers use a set of Unicode blocks for Chinese, Japanese, and Korean writing, such as CJK ideographs, kana, Hangul, and Bopomofo.
The set also includes related historical scripts and text symbols, such as Seal, Nüshu, and Tai Xuan Jing Symbols.
Scripts of other languages, such as Tangut, Khitan Small Script, and Jurchen, are not included.

They check whole blocks, including unassigned code points within those blocks.
Characters in other blocks are not included, even if CJK text often uses them, such as Bopomofo tone marks (ˊ, ˇ, ˋ) and general punctuation (…, —).

These helpers do not detect the language of a character or text and do not query the Unicode Script property.

#### Determine whether a Unicode block is in the CJK set.

```rust
assert!(unicode_blocks::is_cjk_block(unicode_blocks::CJK_UNIFIED_IDEOGRAPHS));
```

#### Determine whether a character belongs to a block in the CJK set.

```rust
assert!(unicode_blocks::is_cjk('。'));
```

## Crates.io

https://crates.io/crates/unicode-blocks

## Documentation

https://docs.rs/unicode-blocks

## License

[MIT](LICENSE)
