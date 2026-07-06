//! 数九（冬至后每九天为一九，共九九八十一天）。

/// 数九。
#[derive(Clone, Debug)]
pub struct ShuJiu {
    name: String,
    index: i32,
}

impl ShuJiu {
    pub(crate) const fn new(name: String, index: i32) -> Self {
        Self { name, index }
    }
    /// 名称（一九 / 二九 …）。
    pub fn name(&self) -> &str {
        &self.name
    }
    /// 该九的第几天（1..9）。
    #[inline]
    pub const fn index(&self) -> i32 {
        self.index
    }
}
