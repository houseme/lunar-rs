//! 数九（冬至后每九天为一九，共九九八十一天）。

use std::fmt;

use crate::culture::{CultureDay, NamedCulture};

/// 数九。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    #[inline]
    pub const fn day_index(&self) -> i32 {
        self.index
    }
}

impl fmt::Display for ShuJiu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name, self.index)
    }
}

impl NamedCulture for ShuJiu {
    fn name(&self) -> &str {
        self.name()
    }
}

impl CultureDay for ShuJiu {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index())
    }
}
