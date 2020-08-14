/*!
# Unicode Blocks

This crate contains a list of all unicode blocks and provides some functions to search across them.

## Examples

#### Given a character, determine what unicode block contains it.

```rust
extern crate unicode_blocks;

assert_eq!(unicode_blocks::BASIC_LATIN, unicode_blocks::find_unicode_block('A').unwrap());
```

#### Given a unicode block, determine whether it is used in CJK.

```rust
extern crate unicode_blocks;

assert!(unicode_blocks::is_cjk_block(unicode_blocks::CJK_UNIFIED_IDEOGRAPHS));
```

#### Given a character, determine whether it is in CJK.

```rust
extern crate unicode_blocks;

assert!(unicode_blocks::is_cjk('。'));
```
*/

#![no_std]

mod cjk;
mod unicode_block;
mod unicode_blocks;

pub use self::unicode_blocks::*;
pub use cjk::*;
pub use unicode_block::*;
