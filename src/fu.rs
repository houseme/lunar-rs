//! 三伏（初伏 / 中伏 / 末伏）。

/// 三伏。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Fu {
    name: String,
    index: i32,
}

impl Fu {
    pub(crate) fn new(name: &str, index: i32) -> Self {
        Self { name: name.to_string(), index }
    }
    /// 名称（初伏 / 中伏 / 末伏）。
    pub fn name(&self) -> &str {
        &self.name
    }
    /// 该伏的第几天。
    #[inline]
    pub const fn index(&self) -> i32 {
        self.index
    }
}
