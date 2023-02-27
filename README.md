# `unicode-blocks`

[![CI](https://github.com/magiclen/unicode-blocks/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/unicode-blocks/actions/workflows/ci.yml)
[![Documentation](https://docs.rs/unicode-blocks/badge.svg)](https://docs.rs/unicode-blocks)
[![Crate](https://img.shields.io/crates/v/unicode-blocks.svg)](https://crates.io/crates/unicode-blocks)

This crate contains a list of all unicode blocks and provides some functions to search across them.

## Features

* `all` – Include an array that contains references to all unicode blocks.

  As this takes quite a bit of memory (approx 2.6k on a 64bit system) this is
  turned off by default.

## Examples

#### Given a character, determine what unicode block contains it.

```rust
assert_eq!(
    unicode_blocks::BASIC_LATIN,
    unicode_blocks::find_unicode_block('A').unwrap()
);
```

#### Given a unicode block, determine whether it is used in CJK.

```rust
assert!(
    unicode_blocks::is_cjk_block(unicode_blocks::CJK_UNIFIED_IDEOGRAPHS)
);
```

#### Given a character, determine whether it is in CJK.

```rust
assert!(unicode_blocks::is_cjk('。'));
```

## License

[MIT](LICENSE)