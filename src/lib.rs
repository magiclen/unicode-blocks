/*!
# Unicode Blocks

This crate contains a list of all unicode blocks and provides some functions
to search across them.

## Features

* `all` – Include an array that contains references to all unicode blocks.

  As this takes quite a bit of memory (approx 2.6k on a 64bit system) this is
  turned off by default.

## Examples

#### Given a character, determine what unicode block contains it.

```
assert_eq!(
    unicode_blocks::BASIC_LATIN,
    unicode_blocks::find_unicode_block('A').unwrap()
);
```

#### Given a unicode block, determine whether it is used in CJK.

```
assert!(
    unicode_blocks::is_cjk_block(unicode_blocks::CJK_UNIFIED_IDEOGRAPHS)
);
```

#### Given a character, determine whether it is in CJK.

```
assert!(unicode_blocks::is_cjk('。'));
```
*/

#![no_std]

#[cfg(feature = "all")]
pub mod all;
mod cjk;
mod unicode_block;
mod unicode_blocks;

pub use cjk::*;
pub use unicode_block::*;

pub use self::unicode_blocks::*;
