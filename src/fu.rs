//! 三伏（初伏 / 中伏 / 末伏）。

use std::fmt;

use crate::culture::{CultureDay, NamedCulture};

/// 三伏。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    #[inline]
    pub const fn day_index(&self) -> i32 {
        self.index
    }
}

impl fmt::Display for Fu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name, self.index)
    }
}

impl NamedCulture for Fu {
    fn name(&self) -> &str {
        self.name()
    }
}

impl CultureDay for Fu {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index())
    }
}
