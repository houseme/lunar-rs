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

impl EventKind {
    pub const fn is_solar(&self) -> bool {
        matches!(self, Self::SolarFestival | Self::SolarOtherFestival | Self::Holiday | Self::JieQi)
    }

    pub const fn is_lunar(&self) -> bool {
        matches!(self, Self::LunarFestival | Self::LunarOtherFestival | Self::Holiday | Self::JieQi)
    }

    pub const fn is_foto(&self) -> bool {
        matches!(self, Self::FotoFestival | Self::FotoOtherFestival)
    }

    pub const fn is_tao(&self) -> bool {
        matches!(self, Self::TaoFestival)
    }
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
    priority: u8,
    source_id: Option<String>,
    is_observed: bool,
}

impl Event {
    pub fn new(kind: EventKind, calendar_kind: CalendarKind, source: EventSource, name: impl Into<String>, solar: Solar) -> Self {
        Self {
            priority: default_priority_for_kind(&kind),
            source_id: None,
            is_observed: false,
            kind,
            calendar_kind,
            source,
            name: name.into(),
            solar,
            detail: None,
        }
    }

    pub fn with_detail(
        kind: EventKind,
        calendar_kind: CalendarKind,
        source: EventSource,
        name: impl Into<String>,
        solar: Solar,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            priority: default_priority_for_kind(&kind),
            source_id: None,
            is_observed: false,
            kind,
            calendar_kind,
            source,
            name: name.into(),
            solar,
            detail: Some(detail.into()),
        }
    }

    pub fn with_meta(
        kind: EventKind,
        calendar_kind: CalendarKind,
        source: EventSource,
        name: impl Into<String>,
        solar: Solar,
        detail: Option<String>,
        priority: u8,
        source_id: Option<String>,
        is_observed: bool,
    ) -> Self {
        Self { kind, calendar_kind, source, name: name.into(), solar, detail, priority, source_id, is_observed }
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

    pub const fn priority(&self) -> u8 {
        self.priority
    }

    pub fn source_id(&self) -> Option<&str> {
        self.source_id.as_deref()
    }

    pub const fn is_observed(&self) -> bool {
        self.is_observed
    }

    pub fn display_text(&self) -> String {
        match &self.detail {
            Some(detail) if !detail.is_empty() => format!("{} ({detail})", self.name),
            _ => self.name.clone(),
        }
    }

    pub fn has_detail(&self) -> bool {
        self.detail.as_deref().is_some_and(|detail| !detail.is_empty())
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

fn event_kind_rank(kind: &EventKind) -> u8 {
    match kind {
        EventKind::JieQi => 10,
        EventKind::Holiday => 20,
        EventKind::SolarFestival => 30,
        EventKind::SolarOtherFestival => 40,
        EventKind::LunarFestival => 50,
        EventKind::LunarOtherFestival => 60,
        EventKind::FotoFestival => 70,
        EventKind::FotoOtherFestival => 80,
        EventKind::TaoFestival => 90,
    }
}

pub fn default_priority_for_kind(kind: &EventKind) -> u8 {
    event_kind_rank(kind)
}

pub fn sort_events(events: &mut [Event]) {
    events.sort_by(|a, b| {
        (
            a.solar.year(),
            a.solar.month(),
            a.solar.day(),
            a.solar.hour(),
            a.solar.minute(),
            a.solar.second(),
            a.priority,
            a.calendar_label(),
            a.name.as_str(),
            a.detail.as_deref().unwrap_or(""),
            a.source_id.as_deref().unwrap_or(""),
        )
            .cmp(&(
                b.solar.year(),
                b.solar.month(),
                b.solar.day(),
                b.solar.hour(),
                b.solar.minute(),
                b.solar.second(),
                b.priority,
                b.calendar_label(),
                b.name.as_str(),
                b.detail.as_deref().unwrap_or(""),
                b.source_id.as_deref().unwrap_or(""),
            ))
    });
}

pub fn dedup_events(events: &mut Vec<Event>) {
    sort_events(events);
    events.dedup();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventQuery<'a> {
    pub calendar_kind: Option<CalendarKind>,
    pub source: Option<EventSource>,
    pub kind: Option<EventKind>,
    pub name_contains: Option<&'a str>,
    pub detail_contains: Option<&'a str>,
}

impl<'a> EventQuery<'a> {
    pub const fn new() -> Self {
        Self { calendar_kind: None, source: None, kind: None, name_contains: None, detail_contains: None }
    }

    pub const fn with_calendar_kind(mut self, calendar_kind: CalendarKind) -> Self {
        self.calendar_kind = Some(calendar_kind);
        self
    }

    pub const fn with_source(mut self, source: EventSource) -> Self {
        self.source = Some(source);
        self
    }

    pub const fn with_kind(mut self, kind: EventKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn with_name_contains(mut self, needle: &'a str) -> Self {
        self.name_contains = Some(needle);
        self
    }

    pub fn with_detail_contains(mut self, needle: &'a str) -> Self {
        self.detail_contains = Some(needle);
        self
    }

    pub fn matches(&self, event: &Event) -> bool {
        if let Some(calendar_kind) = &self.calendar_kind
            && event.calendar_kind() != calendar_kind
        {
            return false;
        }
        if let Some(source) = &self.source
            && event.source() != source
        {
            return false;
        }
        if let Some(kind) = &self.kind
            && event.kind() != kind
        {
            return false;
        }
        if let Some(name_contains) = self.name_contains
            && !event.name().contains(name_contains)
        {
            return false;
        }
        if let Some(detail_contains) = self.detail_contains {
            let Some(detail) = event.detail() else {
                return false;
            };
            if !detail.contains(detail_contains) {
                return false;
            }
        }
        true
    }
}

pub fn filter_events(events: &[Event], query: &EventQuery<'_>) -> Vec<Event> {
    events.iter().filter(|event| query.matches(event)).cloned().collect()
}
