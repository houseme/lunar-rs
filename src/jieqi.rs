//! 节气对象（名称 + 对应阳历时刻）。

use crate::solar::Solar;
use crate::{CalendarKind, Event, EventKind, EventSource};

/// 一个节气 / 节令 / 气令及其阳历时刻。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct JieQi {
    name: String,
    solar: Solar,
}

impl JieQi {
    pub(crate) fn new(name: impl Into<String>, solar: Solar) -> Self {
        Self { name: name.into(), solar }
    }
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub const fn solar(&self) -> Solar {
        self.solar
    }

    /// 节气名称（显式语言版本，需启用 `i18n` feature）。
    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::jieqi(self.name(), language)
    }

    pub fn to_event(&self, calendar_kind: CalendarKind) -> Event {
        Event::with_detail(
            EventKind::JieQi,
            calendar_kind,
            EventSource::JieQi,
            self.name(),
            self.solar,
            format!("at={}", self.solar.to_ymd_hms()),
        )
    }
}
