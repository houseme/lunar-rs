//! 节气对象（名称 + 对应阳历时刻）。

use crate::solar::Solar;

/// 一个节气 / 节令 / 气令及其阳历时刻。
#[derive(Clone, Copy, Debug)]
pub struct JieQi {
    name: &'static str,
    solar: Solar,
}

impl JieQi {
    pub(crate) const fn new(name: &'static str, solar: Solar) -> Self {
        Self { name, solar }
    }
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }
    #[inline]
    pub const fn solar(&self) -> Solar {
        self.solar
    }
}
