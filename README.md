Unicode Blocks
====================

[![CI](https://github.com/magiclen/unicode-blocks/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/unicode-blocks/actions/workflows/ci.yml)

This crate contains a list of all unicode blocks and provides some functions to search across them.

## Examples

#### Given a character, determine what unicode block contains it.

```rust
assert_eq!(unicode_blocks::BASIC_LATIN, unicode_blocks::find_unicode_block('A').unwrap());
```

#### Given a unicode block, determine whether it is used in CJK.

```rust
assert!(unicode_blocks::is_cjk_block(unicode_blocks::CJK_UNIFIED_IDEOGRAPHS));
```

#### Given a character, determine whether it is in CJK.

```rust
assert!(unicode_blocks::is_cjk('。'));
```

## Crates.io

https://crates.io/crates/unicode-blocks

## Documentation

https://docs.rs/unicode-blocks

## License

[MIT](LICENSE)