use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq)]
pub struct UnicodeBlock {
    pub(crate) name: &'static str,
    pub(crate) start: u32,
    pub(crate) end: u32,
}

impl UnicodeBlock {
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub const fn start(&self) -> u32 {
        self.start
    }

    #[inline]
    pub const fn end(&self) -> u32 {
        self.end
    }
}

impl UnicodeBlock {
    /// Given a character, determine whether this unicode block contains it.
    #[inline]
    pub fn contains(&self, c: char) -> bool {
        let u = c as u32;

        u >= self.start && u <= self.end
    }
}

impl PartialEq for UnicodeBlock {
    #[inline]
    fn eq(&self, other: &UnicodeBlock) -> bool {
        self.start.eq(&other.start)
    }
}

impl PartialOrd for UnicodeBlock {
    #[inline]
    fn partial_cmp(&self, other: &UnicodeBlock) -> Option<Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl Ord for UnicodeBlock {
    #[inline]
    fn cmp(&self, other: &UnicodeBlock) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl Hash for UnicodeBlock {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state)
    }
}
