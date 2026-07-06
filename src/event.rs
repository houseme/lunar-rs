//! Unified calendar events.
//!
//! This is the minimal Phase 3 event model that aggregates festivals, holidays
//! and JieQi into a single typed read-only API.

use crate::Solar;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CalendarKind {
    Solar,
    Lunar,
    Foto,
    Tao,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventSource {
    BuiltInFestival,
    BuiltInOtherFestival,
    HolidayData,
    JieQi,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    SolarFestival,
    SolarOtherFestival,
    LunarFestival,
    LunarOtherFestival,
    FotoFestival,
    FotoOtherFestival,
    TaoFestival,
    Holiday,
    JieQi,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    kind: EventKind,
    calendar_kind: CalendarKind,
    source: EventSource,
    name: String,
    solar: Solar,
    detail: Option<String>,
}

impl Event {
    pub fn new(kind: EventKind, calendar_kind: CalendarKind, source: EventSource, name: impl Into<String>, solar: Solar) -> Self {
        Self { kind, calendar_kind, source, name: name.into(), solar, detail: None }
    }

    pub fn with_detail(
        kind: EventKind,
        calendar_kind: CalendarKind,
        source: EventSource,
        name: impl Into<String>,
        solar: Solar,
        detail: impl Into<String>,
    ) -> Self {
        Self { kind, calendar_kind, source, name: name.into(), solar, detail: Some(detail.into()) }
    }

    pub const fn kind(&self) -> &EventKind {
        &self.kind
    }

    pub const fn calendar_kind(&self) -> &CalendarKind {
        &self.calendar_kind
    }

    pub const fn source(&self) -> &EventSource {
        &self.source
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn solar(&self) -> Solar {
        self.solar
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub const fn category_label(&self) -> &'static str {
        match self.kind {
            EventKind::SolarFestival => "solar_festival",
            EventKind::SolarOtherFestival => "solar_other_festival",
            EventKind::LunarFestival => "lunar_festival",
            EventKind::LunarOtherFestival => "lunar_other_festival",
            EventKind::FotoFestival => "foto_festival",
            EventKind::FotoOtherFestival => "foto_other_festival",
            EventKind::TaoFestival => "tao_festival",
            EventKind::Holiday => "holiday",
            EventKind::JieQi => "jieqi",
        }
    }

    pub const fn source_label(&self) -> &'static str {
        match self.source {
            EventSource::BuiltInFestival => "built_in_festival",
            EventSource::BuiltInOtherFestival => "built_in_other_festival",
            EventSource::HolidayData => "holiday_data",
            EventSource::JieQi => "jieqi",
        }
    }

    pub const fn calendar_label(&self) -> &'static str {
        match self.calendar_kind {
            CalendarKind::Solar => "solar",
            CalendarKind::Lunar => "lunar",
            CalendarKind::Foto => "foto",
            CalendarKind::Tao => "tao",
        }
    }
}
