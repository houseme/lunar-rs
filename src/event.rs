//! Unified calendar events.
//!
//! This is the minimal Phase 3 event model that aggregates festivals, holidays
//! and JieQi into a single typed read-only API.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::sync::{Arc, LazyLock, RwLock};

use crate::culture::{CycleItem, EarthBranch, HeavenStem};
use crate::{Holiday, JieQi, Solar, foto::FotoFestival, tao::TaoFestival};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CalendarKind {
    Solar,
    Lunar,
    Foto,
    Tao,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventSource {
    BuiltInFestival,
    BuiltInOtherFestival,
    HolidayData,
    JieQi,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventSourceFamily {
    Festival,
    OtherFestival,
    Observance,
    Seasonal,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    SolarFestival,
    SolarOtherFestival,
    LunarFestival,
    LunarOtherFestival,
    FotoFestival,
    FotoOtherFestival,
    TaoFestival,
    Holiday,
    HolidayPeriod,
    JieQi,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventType {
    SolarDay,
    SolarWeek,
    LunarDay,
    TermDay,
    TermHs,
    TermEb,
}

impl EventType {
    pub const fn from_code(code: usize) -> Option<Self> {
        match code {
            0 => Some(Self::SolarDay),
            1 => Some(Self::SolarWeek),
            2 => Some(Self::LunarDay),
            3 => Some(Self::TermDay),
            4 => Some(Self::TermHs),
            5 => Some(Self::TermEb),
            _ => None,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "公历日期" => Some(Self::SolarDay),
            "几月第几个星期几" => Some(Self::SolarWeek),
            "农历日期" => Some(Self::LunarDay),
            "节气日期" => Some(Self::TermDay),
            "节气天干" => Some(Self::TermHs),
            "节气地支" => Some(Self::TermEb),
            _ => None,
        }
    }

    pub const fn code(&self) -> usize {
        match self {
            Self::SolarDay => 0,
            Self::SolarWeek => 1,
            Self::LunarDay => 2,
            Self::TermDay => 3,
            Self::TermHs => 4,
            Self::TermEb => 5,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::SolarDay => "公历日期",
            Self::SolarWeek => "几月第几个星期几",
            Self::LunarDay => "农历日期",
            Self::TermDay => "节气日期",
            Self::TermHs => "节气天干",
            Self::TermEb => "节气地支",
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl EventKind {
    pub const fn is_solar(&self) -> bool {
        matches!(
            self,
            Self::SolarFestival | Self::SolarOtherFestival | Self::Holiday | Self::HolidayPeriod | Self::JieQi
        )
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventRangeKind {
    Moment,
    FullDay,
    MultiDay,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    kind: EventKind,
    calendar_kind: CalendarKind,
    source: EventSource,
    name: String,
    solar: Solar,
    end_solar: Option<Solar>,
    range_kind: EventRangeKind,
    detail: Option<String>,
    priority: u8,
    source_id: Option<String>,
    is_observed: bool,
    is_primary: bool,
    tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct SolarFestivalEvent {
    solar: Solar,
    name: &'static str,
    other: bool,
}

impl SolarFestivalEvent {
    pub const fn new(solar: Solar, name: &'static str, other: bool) -> Self {
        Self { solar, name, other }
    }

    pub const fn solar(&self) -> Solar {
        self.solar
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn is_other(&self) -> bool {
        self.other
    }

    pub fn to_event(&self) -> Event {
        let kind = if self.other { EventKind::SolarOtherFestival } else { EventKind::SolarFestival };
        let source = if self.other { EventSource::BuiltInOtherFestival } else { EventSource::BuiltInFestival };
        let priority = if self.other { 40 } else { 30 };
        let source_prefix = if self.other { "solar-other" } else { "solar-festival" };
        let primary = !self.other;
        let tag = if self.other { "other_festival" } else { "festival" };
        let source_tag = if self.other { "built_in_other_festival" } else { "built_in_festival" };

        Event::with_meta(
            kind,
            CalendarKind::Solar,
            source,
            self.name,
            self.solar,
            None,
            priority,
            Some(format!("{source_prefix}:{}:{}", self.solar.to_ymd(), self.name)),
            true,
            primary,
            vec!["solar".to_string(), tag.to_string(), source_tag.to_string()],
        )
    }
}

#[derive(Clone, Debug)]
pub struct LunarFestivalEvent {
    solar: Solar,
    name: &'static str,
    other: bool,
}

impl LunarFestivalEvent {
    pub const fn new(solar: Solar, name: &'static str, other: bool) -> Self {
        Self { solar, name, other }
    }

    pub fn to_event(&self) -> Event {
        let kind = if self.other { EventKind::LunarOtherFestival } else { EventKind::LunarFestival };
        let source = if self.other { EventSource::BuiltInOtherFestival } else { EventSource::BuiltInFestival };
        let priority = if self.other { 60 } else { 50 };
        let source_prefix = if self.other { "lunar-other" } else { "lunar-festival" };
        let primary = !self.other;
        let tag = if self.other { "other_festival" } else { "festival" };
        let source_tag = if self.other { "built_in_other_festival" } else { "built_in_festival" };

        Event::with_meta(
            kind,
            CalendarKind::Lunar,
            source,
            self.name,
            self.solar,
            None,
            priority,
            Some(format!("{source_prefix}:{}:{}", self.solar.to_ymd(), self.name)),
            true,
            primary,
            vec!["lunar".to_string(), tag.to_string(), source_tag.to_string()],
        )
    }
}

#[derive(Clone, Debug)]
pub struct JieQiEvent {
    jieqi: JieQi,
    calendar_kind: CalendarKind,
}

impl JieQiEvent {
    pub const fn new(jieqi: JieQi, calendar_kind: CalendarKind) -> Self {
        Self { jieqi, calendar_kind }
    }

    pub fn to_event(&self) -> Event {
        self.jieqi.to_event(self.calendar_kind)
    }
}

#[derive(Clone, Debug)]
pub struct HolidayEvent {
    holiday: Holiday,
    solar: Solar,
    calendar_kind: CalendarKind,
}

impl HolidayEvent {
    pub const fn new(holiday: Holiday, solar: Solar, calendar_kind: CalendarKind) -> Self {
        Self { holiday, solar, calendar_kind }
    }

    pub fn to_event(&self) -> Event {
        self.holiday.to_event(self.solar, self.calendar_kind)
    }
}

#[derive(Clone, Debug)]
pub struct HolidayPeriodEvent {
    name: String,
    start: Solar,
    end: Solar,
    target: String,
}

impl HolidayPeriodEvent {
    pub fn new(name: impl Into<String>, start: Solar, end: Solar, target: impl Into<String>) -> Self {
        Self { name: name.into(), start, end, target: target.into() }
    }

    pub fn to_event(&self) -> Event {
        self.to_anchored_event(self.start)
    }

    pub fn to_anchored_event(&self, anchor: Solar) -> Event {
        Event::with_meta(
            EventKind::HolidayPeriod,
            CalendarKind::Solar,
            EventSource::HolidayData,
            self.name.clone(),
            anchor,
            Some(format!("target={} range={}->{}", self.target, self.start.to_ymd(), self.end.to_ymd())),
            15,
            Some(format!("holiday-period:{}:{}:{}", self.start.to_ymd(), self.end.to_ymd(), self.name)),
            true,
            true,
            vec![
                "holiday".to_string(),
                "holiday_period".to_string(),
                "observance".to_string(),
                "day_off".to_string(),
                "solar".to_string(),
            ],
        )
        .with_range(EventRangeKind::MultiDay, Some(self.end))
    }
}

#[derive(Clone, Debug)]
pub struct FotoFestivalEvent {
    festival: FotoFestival,
    solar: Solar,
}

impl FotoFestivalEvent {
    pub const fn new(festival: FotoFestival, solar: Solar) -> Self {
        Self { festival, solar }
    }

    pub fn to_event(&self) -> Event {
        self.festival.to_event(self.solar)
    }
}

#[derive(Clone, Debug)]
pub struct TaoFestivalEvent {
    festival: TaoFestival,
    solar: Solar,
}

impl TaoFestivalEvent {
    pub const fn new(festival: TaoFestival, solar: Solar) -> Self {
        Self { festival, solar }
    }

    pub fn to_event(&self) -> Event {
        self.festival.to_event(self.solar)
    }
}

impl Event {
    pub fn new(
        kind: EventKind,
        calendar_kind: CalendarKind,
        source: EventSource,
        name: impl Into<String>,
        solar: Solar,
    ) -> Self {
        Self {
            priority: default_priority_for_kind(&kind),
            source_id: None,
            is_observed: false,
            is_primary: default_primary_for_kind(&kind),
            tags: default_tags(&kind, &calendar_kind, &source),
            kind,
            calendar_kind,
            source,
            name: name.into(),
            solar,
            end_solar: None,
            range_kind: default_range_kind_for_kind(&kind),
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
            is_primary: default_primary_for_kind(&kind),
            tags: default_tags(&kind, &calendar_kind, &source),
            kind,
            calendar_kind,
            source,
            name: name.into(),
            solar,
            end_solar: None,
            range_kind: default_range_kind_for_kind(&kind),
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
        is_primary: bool,
        tags: Vec<String>,
    ) -> Self {
        Self {
            kind,
            calendar_kind,
            source,
            name: name.into(),
            solar,
            end_solar: None,
            range_kind: default_range_kind_for_kind(&kind),
            detail,
            priority,
            source_id,
            is_observed,
            is_primary,
            tags,
        }
    }

    pub fn with_range(mut self, range_kind: EventRangeKind, end_solar: Option<Solar>) -> Self {
        self.range_kind = range_kind;
        self.end_solar = end_solar;
        self
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

    pub const fn end_solar(&self) -> Option<Solar> {
        self.end_solar
    }

    pub const fn range_kind(&self) -> EventRangeKind {
        self.range_kind
    }

    pub const fn source_family(&self) -> EventSourceFamily {
        match self.kind {
            EventKind::SolarFestival | EventKind::LunarFestival | EventKind::FotoFestival | EventKind::TaoFestival => {
                EventSourceFamily::Festival
            }
            EventKind::SolarOtherFestival | EventKind::LunarOtherFestival | EventKind::FotoOtherFestival => {
                EventSourceFamily::OtherFestival
            }
            EventKind::Holiday | EventKind::HolidayPeriod => EventSourceFamily::Observance,
            EventKind::JieQi => EventSourceFamily::Seasonal,
        }
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

    pub const fn is_primary(&self) -> bool {
        self.is_primary
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|value| value == tag)
    }

    pub fn spans_multiple_days(&self) -> bool {
        self.end_solar.is_some_and(|end| {
            (self.solar.year(), self.solar.month(), self.solar.day()) != (end.year(), end.month(), end.day())
        })
    }

    pub fn covers_solar(&self, solar: Solar) -> bool {
        match self.range_kind {
            EventRangeKind::Moment => solar == self.solar,
            EventRangeKind::FullDay => {
                (self.solar.year(), self.solar.month(), self.solar.day()) == (solar.year(), solar.month(), solar.day())
            }
            EventRangeKind::MultiDay => {
                if same_calendar_day(solar, self.solar) {
                    return true;
                }
                self.end_solar.is_some_and(|end| !solar.is_before(&self.solar) && !solar.is_after(&end))
            }
        }
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
            EventKind::HolidayPeriod => "holiday_period",
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
        EventKind::HolidayPeriod => 15,
        EventKind::SolarFestival => 30,
        EventKind::SolarOtherFestival => 40,
        EventKind::LunarFestival => 50,
        EventKind::LunarOtherFestival => 60,
        EventKind::FotoFestival => 70,
        EventKind::FotoOtherFestival => 80,
        EventKind::TaoFestival => 90,
    }
}

pub const fn default_range_kind_for_kind(kind: &EventKind) -> EventRangeKind {
    match kind {
        EventKind::JieQi => EventRangeKind::Moment,
        EventKind::HolidayPeriod => EventRangeKind::MultiDay,
        _ => EventRangeKind::FullDay,
    }
}

pub fn default_priority_for_kind(kind: &EventKind) -> u8 {
    event_kind_rank(kind)
}

pub const fn default_primary_for_kind(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::JieQi
            | EventKind::Holiday
            | EventKind::HolidayPeriod
            | EventKind::SolarFestival
            | EventKind::LunarFestival
            | EventKind::FotoFestival
            | EventKind::TaoFestival
    )
}

fn default_tags(kind: &EventKind, calendar_kind: &CalendarKind, source: &EventSource) -> Vec<String> {
    let mut tags = vec![
        calendar_kind_label(calendar_kind).to_string(),
        event_kind_label(kind).to_string(),
        source_label(source).to_string(),
    ];

    match kind {
        EventKind::Holiday => tags.push("observance".to_string()),
        EventKind::HolidayPeriod => {
            tags.push("observance".to_string());
            tags.push("holiday_period".to_string());
        }
        EventKind::JieQi => tags.push("seasonal".to_string()),
        EventKind::SolarFestival | EventKind::LunarFestival | EventKind::FotoFestival | EventKind::TaoFestival => {
            tags.push("festival".to_string());
        }
        EventKind::SolarOtherFestival | EventKind::LunarOtherFestival | EventKind::FotoOtherFestival => {
            tags.push("other_festival".to_string());
        }
    }

    tags
}

const fn calendar_kind_label(calendar_kind: &CalendarKind) -> &'static str {
    match calendar_kind {
        CalendarKind::Solar => "solar",
        CalendarKind::Lunar => "lunar",
        CalendarKind::Foto => "foto",
        CalendarKind::Tao => "tao",
    }
}

const fn source_label(source: &EventSource) -> &'static str {
    match source {
        EventSource::BuiltInFestival => "built_in_festival",
        EventSource::BuiltInOtherFestival => "built_in_other_festival",
        EventSource::HolidayData => "holiday_data",
        EventSource::JieQi => "jieqi",
    }
}

const fn event_kind_label(kind: &EventKind) -> &'static str {
    match kind {
        EventKind::SolarFestival => "solar_festival",
        EventKind::SolarOtherFestival => "solar_other_festival",
        EventKind::LunarFestival => "lunar_festival",
        EventKind::LunarOtherFestival => "lunar_other_festival",
        EventKind::FotoFestival => "foto_festival",
        EventKind::FotoOtherFestival => "foto_other_festival",
        EventKind::TaoFestival => "tao_festival",
        EventKind::Holiday => "holiday",
        EventKind::HolidayPeriod => "holiday_period",
        EventKind::JieQi => "jieqi",
    }
}

fn same_calendar_day(a: Solar, b: Solar) -> bool {
    (a.year(), a.month(), a.day()) == (b.year(), b.month(), b.day())
}

pub fn sort_events(events: &mut [Event]) {
    events.sort_by(|a, b| {
        (a.solar.year(), a.solar.month(), a.solar.day(), a.solar.hour(), a.solar.minute(), a.solar.second())
            .cmp(&(b.solar.year(), b.solar.month(), b.solar.day(), b.solar.hour(), b.solar.minute(), b.solar.second()))
            .then_with(|| {
                (
                    a.end_solar.map_or(i32::MIN, |value| value.year()),
                    a.end_solar.map_or(i32::MIN, |value| value.month()),
                    a.end_solar.map_or(i32::MIN, |value| value.day()),
                    a.end_solar.map_or(i32::MIN, |value| value.hour()),
                    a.end_solar.map_or(i32::MIN, |value| value.minute()),
                    a.end_solar.map_or(i32::MIN, |value| value.second()),
                )
                    .cmp(&(
                        b.end_solar.map_or(i32::MIN, |value| value.year()),
                        b.end_solar.map_or(i32::MIN, |value| value.month()),
                        b.end_solar.map_or(i32::MIN, |value| value.day()),
                        b.end_solar.map_or(i32::MIN, |value| value.hour()),
                        b.end_solar.map_or(i32::MIN, |value| value.minute()),
                        b.end_solar.map_or(i32::MIN, |value| value.second()),
                    ))
            })
            .then_with(|| event_range_rank(&a.range_kind).cmp(&event_range_rank(&b.range_kind)))
            .then_with(|| a.priority.cmp(&b.priority))
            .then_with(|| a.calendar_label().cmp(b.calendar_label()))
            .then_with(|| a.name.as_str().cmp(b.name.as_str()))
            .then_with(|| a.detail.as_deref().unwrap_or("").cmp(b.detail.as_deref().unwrap_or("")))
            .then_with(|| a.source_id.as_deref().unwrap_or("").cmp(b.source_id.as_deref().unwrap_or("")))
    });
}

const fn event_range_rank(range_kind: &EventRangeKind) -> u8 {
    match range_kind {
        EventRangeKind::Moment => 0,
        EventRangeKind::FullDay => 1,
        EventRangeKind::MultiDay => 2,
    }
}

pub fn dedup_events(events: &mut Vec<Event>) {
    sort_events(events);
    events.dedup();
}

#[derive(Clone, Debug, Default)]
struct EventIndex {
    events: Vec<Event>,
    by_calendar_kind: HashMap<CalendarKind, Vec<usize>>,
    by_source: HashMap<EventSource, Vec<usize>>,
    by_source_family: HashMap<EventSourceFamily, Vec<usize>>,
    by_kind: HashMap<EventKind, Vec<usize>>,
    by_range_kind: HashMap<EventRangeKind, Vec<usize>>,
    by_is_primary: HashMap<bool, Vec<usize>>,
    by_is_observed: HashMap<bool, Vec<usize>>,
    by_tag: HashMap<String, Vec<usize>>,
}

impl EventIndex {
    fn new(mut events: Vec<Event>) -> Self {
        dedup_events(&mut events);

        let mut index = Self {
            events,
            by_calendar_kind: HashMap::new(),
            by_source: HashMap::new(),
            by_source_family: HashMap::new(),
            by_kind: HashMap::new(),
            by_range_kind: HashMap::new(),
            by_is_primary: HashMap::new(),
            by_is_observed: HashMap::new(),
            by_tag: HashMap::new(),
        };

        for (position, event) in index.events.iter().enumerate() {
            index.by_calendar_kind.entry(*event.calendar_kind()).or_default().push(position);
            index.by_source.entry(*event.source()).or_default().push(position);
            index.by_source_family.entry(event.source_family()).or_default().push(position);
            index.by_kind.entry(*event.kind()).or_default().push(position);
            index.by_range_kind.entry(event.range_kind()).or_default().push(position);
            index.by_is_primary.entry(event.is_primary()).or_default().push(position);
            index.by_is_observed.entry(event.is_observed()).or_default().push(position);
            for tag in event.tags() {
                index.by_tag.entry(tag.clone()).or_default().push(position);
            }
        }

        index
    }

    fn events(&self) -> &[Event] {
        &self.events
    }

    fn filter(&self, query: &EventQuery<'_>) -> Vec<Event> {
        let candidate_positions = [
            query.calendar_kind.and_then(|value| self.by_calendar_kind.get(&value)),
            query.source.and_then(|value| self.by_source.get(&value)),
            query.source_family.and_then(|value| self.by_source_family.get(&value)),
            query.kind.and_then(|value| self.by_kind.get(&value)),
            query.range_kind.and_then(|value| self.by_range_kind.get(&value)),
            query.is_primary.and_then(|value| self.by_is_primary.get(&value)),
            query.is_observed.and_then(|value| self.by_is_observed.get(&value)),
            query.has_tag.and_then(|value| self.by_tag.get(value)),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|positions| positions.len());

        match candidate_positions {
            Some(positions) => positions
                .iter()
                .filter_map(|&position| {
                    let event = &self.events[position];
                    query.matches(event).then(|| event.clone())
                })
                .collect(),
            None => self.events.iter().filter(|event| query.matches(event)).cloned().collect(),
        }
    }
}

static EVENT_INDEX_CACHE: LazyLock<RwLock<HashMap<Solar, Arc<EventIndex>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static EVENT_MANAGER_RULES: LazyLock<RwLock<BTreeMap<String, EventRule>>> =
    LazyLock::new(|| RwLock::new(BTreeMap::new()));

pub(crate) fn clear_event_index_cache() {
    EVENT_INDEX_CACHE.write().unwrap().clear();
}

fn build_all_events_for_day(solar: Solar) -> Vec<Event> {
    let mut events = solar.events();
    events.extend(holiday_period_events_for_day(solar));
    events.extend(EventManager::events_for_day(solar));
    let lunar = solar.lunar();
    events.extend(
        lunar
            .events()
            .into_iter()
            .filter(|event| matches!(event.kind(), EventKind::LunarFestival | EventKind::LunarOtherFestival)),
    );
    events.extend(lunar.foto().events());
    events.extend(lunar.tao().events());
    dedup_events(&mut events);
    events
}

pub(crate) fn holiday_period_events_for_day(solar: Solar) -> Vec<Event> {
    let mut records = Vec::new();
    for year in [solar.year() - 1, solar.year(), solar.year() + 1] {
        records.extend(crate::holiday_util::get_holidays_by_year(year));
    }

    let mut by_key: BTreeMap<(String, String), Vec<Holiday>> = BTreeMap::new();
    for holiday in records {
        if holiday.is_work() {
            continue;
        }
        by_key.entry((holiday.name().to_string(), holiday.target().to_string())).or_default().push(holiday);
    }

    let mut events = Vec::new();
    for ((name, target), mut holidays) in by_key {
        holidays.sort_by(|a, b| a.day().cmp(b.day()));
        let mut index = 0;
        while index < holidays.len() {
            let start_day = holidays[index].day().to_string();
            let start = solar_from_ymd(&start_day);
            let mut end = start;
            let mut cursor = index + 1;
            while cursor < holidays.len() {
                let next = solar_from_ymd(holidays[cursor].day());
                if next.subtract(&end) != 1 {
                    break;
                }
                end = next;
                cursor += 1;
            }

            if cursor - index > 1 && !solar.is_before(&start) && !solar.is_after(&end) {
                events.push(HolidayPeriodEvent::new(name.clone(), start, end, target.clone()).to_anchored_event(solar));
            }

            index = cursor;
        }
    }
    events
}

fn solar_from_ymd(ymd: &str) -> Solar {
    let year: i32 = ymd[0..4].parse().unwrap_or(0);
    let month: i32 = ymd[5..7].parse().unwrap_or(1);
    let day: i32 = ymd[8..10].parse().unwrap_or(1);
    Solar::from_ymd(year, month, day).unwrap()
}

fn day_event_index(solar: Solar) -> Arc<EventIndex> {
    {
        let cache = EVENT_INDEX_CACHE.read().unwrap();
        if let Some(index) = cache.get(&solar) {
            return Arc::clone(index);
        }
    }

    let index = Arc::new(EventIndex::new(build_all_events_for_day(solar)));
    let mut cache = EVENT_INDEX_CACHE.write().unwrap();
    Arc::clone(cache.entry(solar).or_insert_with(|| Arc::clone(&index)))
}

pub(crate) fn all_events_for_day(solar: Solar) -> Vec<Event> {
    day_event_index(solar).events().to_vec()
}

pub(crate) fn find_events_for_day(solar: Solar, query: &EventQuery<'_>) -> Vec<Event> {
    day_event_index(solar).filter(query)
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventRule {
    SolarDay {
        month: i32,
        day: i32,
        offset_days: i32,
        start_year: i32,
    },
    LunarDay {
        month: i32,
        day: i32,
        offset_days: i32,
        start_year: i32,
    },
    SolarWeek {
        month: i32,
        week_index: i32,
        week: i32,
        offset_days: i32,
        start_year: i32,
    },
    SolarTermOffset {
        term: String,
        offset_days: i32,
        start_year: i32,
    },
    SolarTermHeavenStem {
        term: String,
        heaven_stem_index: usize,
        search_start_offset_days: i32,
        offset_days: i32,
        start_year: i32,
    },
    SolarTermEarthBranch {
        term: String,
        earth_branch_index: usize,
        search_start_offset_days: i32,
        offset_days: i32,
        start_year: i32,
    },
}

impl EventRule {
    pub const fn solar_day(month: i32, day: i32) -> Self {
        Self::SolarDay { month, day, offset_days: 0, start_year: 0 }
    }

    pub const fn lunar_day(month: i32, day: i32) -> Self {
        Self::LunarDay { month, day, offset_days: 0, start_year: 0 }
    }

    pub const fn solar_week(month: i32, week_index: i32, week: i32) -> Self {
        Self::SolarWeek { month, week_index, week, offset_days: 0, start_year: 0 }
    }

    pub fn solar_term_offset(term: impl Into<String>, offset_days: i32) -> Self {
        Self::SolarTermOffset { term: term.into(), offset_days, start_year: 0 }
    }

    pub fn solar_term_heaven_stem(
        term: impl Into<String>,
        heaven_stem_index: usize,
        search_start_offset_days: i32,
    ) -> Self {
        Self::SolarTermHeavenStem {
            term: term.into(),
            heaven_stem_index,
            search_start_offset_days,
            offset_days: 0,
            start_year: 0,
        }
    }

    pub fn solar_term_earth_branch(
        term: impl Into<String>,
        earth_branch_index: usize,
        search_start_offset_days: i32,
    ) -> Self {
        Self::SolarTermEarthBranch {
            term: term.into(),
            earth_branch_index,
            search_start_offset_days,
            offset_days: 0,
            start_year: 0,
        }
    }

    pub fn with_offset_days(self, offset_days: i32) -> Self {
        match self {
            Self::SolarDay { month, day, start_year, .. } => Self::SolarDay { month, day, offset_days, start_year },
            Self::LunarDay { month, day, start_year, .. } => Self::LunarDay { month, day, offset_days, start_year },
            Self::SolarWeek { month, week_index, week, start_year, .. } => {
                Self::SolarWeek { month, week_index, week, offset_days, start_year }
            }
            Self::SolarTermOffset { term, start_year, .. } => Self::SolarTermOffset { term, offset_days, start_year },
            Self::SolarTermHeavenStem { term, heaven_stem_index, search_start_offset_days, start_year, .. } => {
                Self::SolarTermHeavenStem { term, heaven_stem_index, search_start_offset_days, offset_days, start_year }
            }
            Self::SolarTermEarthBranch { term, earth_branch_index, search_start_offset_days, start_year, .. } => {
                Self::SolarTermEarthBranch {
                    term,
                    earth_branch_index,
                    search_start_offset_days,
                    offset_days,
                    start_year,
                }
            }
        }
    }

    pub fn with_start_year(self, start_year: i32) -> Self {
        match self {
            Self::SolarDay { month, day, offset_days, .. } => Self::SolarDay { month, day, offset_days, start_year },
            Self::LunarDay { month, day, offset_days, .. } => Self::LunarDay { month, day, offset_days, start_year },
            Self::SolarWeek { month, week_index, week, offset_days, .. } => {
                Self::SolarWeek { month, week_index, week, offset_days, start_year }
            }
            Self::SolarTermOffset { term, offset_days, .. } => Self::SolarTermOffset { term, offset_days, start_year },
            Self::SolarTermHeavenStem { term, heaven_stem_index, search_start_offset_days, offset_days, .. } => {
                Self::SolarTermHeavenStem { term, heaven_stem_index, search_start_offset_days, offset_days, start_year }
            }
            Self::SolarTermEarthBranch { term, earth_branch_index, search_start_offset_days, offset_days, .. } => {
                Self::SolarTermEarthBranch {
                    term,
                    earth_branch_index,
                    search_start_offset_days,
                    offset_days,
                    start_year,
                }
            }
        }
    }

    pub fn resolve_solar(&self, year: i32) -> Option<Solar> {
        if year < self.start_year() {
            return None;
        }

        let solar = match self {
            Self::SolarDay { month, day, .. } => Solar::from_ymd(year, *month, *day).ok(),
            Self::LunarDay { month, day, .. } => {
                crate::Lunar::from_ymd(year, *month, *day).ok().map(|lunar| lunar.solar())
            }
            Self::SolarWeek { month, week_index, week, .. } => resolve_solar_week(year, *month, *week_index, *week),
            Self::SolarTermOffset { term, .. } => resolve_solar_term(year, term),
            Self::SolarTermHeavenStem { term, heaven_stem_index, search_start_offset_days, .. } => {
                resolve_solar_term_heaven_stem(year, term, *heaven_stem_index, *search_start_offset_days)
            }
            Self::SolarTermEarthBranch { term, earth_branch_index, search_start_offset_days, .. } => {
                resolve_solar_term_earth_branch(year, term, *earth_branch_index, *search_start_offset_days)
            }
        }?;

        Some(solar.next_day(self.offset_days()))
    }

    pub fn to_event(&self, name: impl Into<String>, year: i32) -> Option<Event> {
        let name = name.into();
        let solar = self.resolve_solar(year)?;
        Some(rule_event(name, solar, self))
    }

    pub const fn offset_days(&self) -> i32 {
        match self {
            Self::SolarDay { offset_days, .. }
            | Self::LunarDay { offset_days, .. }
            | Self::SolarWeek { offset_days, .. }
            | Self::SolarTermOffset { offset_days, .. }
            | Self::SolarTermHeavenStem { offset_days, .. }
            | Self::SolarTermEarthBranch { offset_days, .. } => *offset_days,
        }
    }

    pub const fn start_year(&self) -> i32 {
        match self {
            Self::SolarDay { start_year, .. }
            | Self::LunarDay { start_year, .. }
            | Self::SolarWeek { start_year, .. }
            | Self::SolarTermOffset { start_year, .. }
            | Self::SolarTermHeavenStem { start_year, .. }
            | Self::SolarTermEarthBranch { start_year, .. } => *start_year,
        }
    }

    pub const fn kind_label(&self) -> &'static str {
        match self {
            Self::SolarDay { .. } => "solar_day",
            Self::LunarDay { .. } => "lunar_day",
            Self::SolarWeek { .. } => "solar_week",
            Self::SolarTermOffset { .. } => "solar_term_offset",
            Self::SolarTermHeavenStem { .. } => "solar_term_heaven_stem",
            Self::SolarTermEarthBranch { .. } => "solar_term_earth_branch",
        }
    }

    pub const fn event_type(&self) -> EventType {
        match self {
            Self::SolarDay { .. } => EventType::SolarDay,
            Self::LunarDay { .. } => EventType::LunarDay,
            Self::SolarWeek { .. } => EventType::SolarWeek,
            Self::SolarTermOffset { .. } => EventType::TermDay,
            Self::SolarTermHeavenStem { .. } => EventType::TermHs,
            Self::SolarTermEarthBranch { .. } => EventType::TermEb,
        }
    }
}

fn resolve_solar_week(year: i32, month: i32, week_index: i32, week: i32) -> Option<Solar> {
    if week_index == 0 || !(0..=6).contains(&week) {
        return None;
    }

    if week_index > 0 {
        let first = Solar::from_ymd(year, month, 1).ok()?;
        let days = (week - first.week()).rem_euclid(7) + 7 * (week_index - 1);
        let solar = first.next_day(days);
        return (solar.month() == month).then_some(solar);
    }

    let last_day = crate::solar_util::days_of_month(year, month);
    let last = Solar::from_ymd(year, month, last_day).ok()?;
    let days = -((last.week() - week).rem_euclid(7)) + 7 * (week_index + 1);
    let solar = last.next_day(days);
    (solar.month() == month).then_some(solar)
}

fn resolve_solar_term(year: i32, term: &str) -> Option<Solar> {
    let anchor = Solar::from_ymd(year, 7, 1).ok()?.lunar();
    anchor.jie_qi_table().get(term).copied()
}

fn resolve_solar_term_heaven_stem(
    year: i32,
    term: &str,
    heaven_stem_index: usize,
    search_start_offset_days: i32,
) -> Option<Solar> {
    let start = resolve_solar_term(year, term)?.next_day(search_start_offset_days);
    let current = start.lunar().day_sixty_cycle().heaven_stem();
    let offset = current.steps_to(HeavenStem::from_index(heaven_stem_index % HeavenStem::size()).index());
    Some(start.next_day(offset as i32))
}

fn resolve_solar_term_earth_branch(
    year: i32,
    term: &str,
    earth_branch_index: usize,
    search_start_offset_days: i32,
) -> Option<Solar> {
    let start = resolve_solar_term(year, term)?.next_day(search_start_offset_days);
    let current = start.lunar().day_sixty_cycle().earth_branch();
    let offset = current.steps_to(EarthBranch::from_index(earth_branch_index % EarthBranch::size()).index());
    Some(start.next_day(offset as i32))
}

fn rule_event(name: String, solar: Solar, rule: &EventRule) -> Event {
    Event::with_meta(
        EventKind::SolarOtherFestival,
        CalendarKind::Solar,
        EventSource::BuiltInOtherFestival,
        name.clone(),
        solar,
        Some(format!("rule={} offset_days={}", rule.kind_label(), rule.offset_days())),
        35,
        Some(format!("event-manager:{}:{}:{}", rule.kind_label(), solar.to_ymd(), name)),
        true,
        true,
        vec![
            "event_manager".to_string(),
            "rule".to_string(),
            "solar".to_string(),
            "other_festival".to_string(),
            "built_in_other_festival".to_string(),
        ],
    )
}

pub struct EventManager;

impl EventManager {
    pub fn update(name: impl Into<String>, rule: EventRule) {
        EVENT_MANAGER_RULES.write().unwrap().insert(name.into(), rule);
        clear_event_index_cache();
    }

    pub fn remove(name: &str) -> Option<EventRule> {
        let removed = EVENT_MANAGER_RULES.write().unwrap().remove(name);
        if removed.is_some() {
            clear_event_index_cache();
        }
        removed
    }

    pub fn clear() {
        EVENT_MANAGER_RULES.write().unwrap().clear();
        clear_event_index_cache();
    }

    pub fn rules() -> Vec<(String, EventRule)> {
        EVENT_MANAGER_RULES.read().unwrap().iter().map(|(name, rule)| (name.clone(), rule.clone())).collect()
    }

    pub fn event_for_year(name: &str, year: i32) -> Option<Event> {
        EVENT_MANAGER_RULES.read().unwrap().get(name)?.to_event(name.to_string(), year)
    }

    pub fn events_for_day(solar: Solar) -> Vec<Event> {
        let rules = EVENT_MANAGER_RULES.read().unwrap().clone();
        rules
            .into_iter()
            .filter_map(|(name, rule)| rule.to_event(name, solar.year()))
            .filter(|event| event.covers_solar(solar))
            .collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventQuery<'a> {
    pub calendar_kind: Option<CalendarKind>,
    pub source: Option<EventSource>,
    pub source_family: Option<EventSourceFamily>,
    pub kind: Option<EventKind>,
    pub range_kind: Option<EventRangeKind>,
    pub is_primary: Option<bool>,
    pub is_observed: Option<bool>,
    pub name_contains: Option<&'a str>,
    pub detail_contains: Option<&'a str>,
    pub has_tag: Option<&'a str>,
}

impl<'a> EventQuery<'a> {
    pub const fn new() -> Self {
        Self {
            calendar_kind: None,
            source: None,
            source_family: None,
            kind: None,
            range_kind: None,
            is_primary: None,
            is_observed: None,
            name_contains: None,
            detail_contains: None,
            has_tag: None,
        }
    }

    pub const fn with_calendar_kind(mut self, calendar_kind: CalendarKind) -> Self {
        self.calendar_kind = Some(calendar_kind);
        self
    }

    pub const fn with_source(mut self, source: EventSource) -> Self {
        self.source = Some(source);
        self
    }

    pub const fn with_source_family(mut self, source_family: EventSourceFamily) -> Self {
        self.source_family = Some(source_family);
        self
    }

    pub const fn with_kind(mut self, kind: EventKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub const fn with_range_kind(mut self, range_kind: EventRangeKind) -> Self {
        self.range_kind = Some(range_kind);
        self
    }

    pub const fn with_is_primary(mut self, is_primary: bool) -> Self {
        self.is_primary = Some(is_primary);
        self
    }

    pub const fn with_is_observed(mut self, is_observed: bool) -> Self {
        self.is_observed = Some(is_observed);
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

    pub fn with_tag(mut self, tag: &'a str) -> Self {
        self.has_tag = Some(tag);
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
        if let Some(source_family) = &self.source_family
            && event.source_family() != *source_family
        {
            return false;
        }
        if let Some(kind) = &self.kind
            && event.kind() != kind
        {
            return false;
        }
        if let Some(range_kind) = &self.range_kind
            && event.range_kind() != *range_kind
        {
            return false;
        }
        if let Some(is_primary) = self.is_primary
            && event.is_primary() != is_primary
        {
            return false;
        }
        if let Some(is_observed) = self.is_observed
            && event.is_observed() != is_observed
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
        if let Some(tag) = self.has_tag
            && !event.has_tag(tag)
        {
            return false;
        }
        true
    }
}

pub fn filter_events(events: &[Event], query: &EventQuery<'_>) -> Vec<Event> {
    events.iter().filter(|event| query.matches(event)).cloned().collect()
}

pub fn scan_events_in_range(start: Solar, end: Solar) -> Vec<Event> {
    let mut events = Vec::new();
    if start.is_after(&end) {
        return events;
    }

    let mut cursor = start;
    loop {
        events.extend(day_event_index(cursor).events().iter().cloned());
        if cursor == end {
            break;
        }
        cursor = cursor.next_day(1);
    }
    events
}

pub fn scan_events_in_range_filtered(start: Solar, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
    let mut events = Vec::new();
    if start.is_after(&end) {
        return events;
    }

    let mut cursor = start;
    loop {
        events.extend(day_event_index(cursor).filter(query));
        if cursor == end {
            break;
        }
        cursor = cursor.next_day(1);
    }

    events
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventDayGroup {
    solar: Solar,
    events: Vec<Event>,
}

impl EventDayGroup {
    pub fn new(solar: Solar, mut events: Vec<Event>) -> Self {
        dedup_events(&mut events);
        Self { solar, events }
    }

    pub const fn solar(&self) -> Solar {
        self.solar
    }

    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

pub fn group_events_by_day(mut events: Vec<Event>) -> Vec<EventDayGroup> {
    dedup_events(&mut events);
    let mut groups: Vec<EventDayGroup> = Vec::new();

    for event in events {
        if let Some(group) = groups.last_mut()
            && (group.solar.year(), group.solar.month(), group.solar.day())
                == (event.solar().year(), event.solar().month(), event.solar().day())
        {
            group.events.push(event);
            continue;
        }
        groups.push(EventDayGroup::new(event.solar(), vec![event]));
    }

    groups
}

pub fn scan_event_days_in_range(start: Solar, end: Solar) -> Vec<EventDayGroup> {
    let mut groups = Vec::new();
    if start.is_after(&end) {
        return groups;
    }

    let mut cursor = start;
    loop {
        let events = day_event_index(cursor).events().to_vec();
        if !events.is_empty() {
            groups.push(EventDayGroup::new(cursor, events));
        }
        if cursor == end {
            break;
        }
        cursor = cursor.next_day(1);
    }

    groups
}

pub fn scan_event_days_in_range_filtered(start: Solar, end: Solar, query: &EventQuery<'_>) -> Vec<EventDayGroup> {
    let mut groups = Vec::new();
    if start.is_after(&end) {
        return groups;
    }

    let mut cursor = start;
    loop {
        let events = day_event_index(cursor).filter(query);
        if !events.is_empty() {
            groups.push(EventDayGroup::new(cursor, events));
        }
        if cursor == end {
            break;
        }
        cursor = cursor.next_day(1);
    }

    groups
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventWeekGroup {
    start: Solar,
    end: Solar,
    days: Vec<EventDayGroup>,
}

impl EventWeekGroup {
    pub fn new(start: Solar, end: Solar, days: Vec<EventDayGroup>) -> Self {
        Self { start, end, days }
    }

    pub const fn start(&self) -> Solar {
        self.start
    }

    pub const fn end(&self) -> Solar {
        self.end
    }

    pub fn days(&self) -> &[EventDayGroup] {
        &self.days
    }
}

pub fn group_event_days_by_week(days: Vec<EventDayGroup>, week_start: i32) -> Vec<EventWeekGroup> {
    let mut groups = Vec::new();
    let mut current_start: Option<Solar> = None;
    let mut current_end: Option<Solar> = None;
    let mut current_days: Vec<EventDayGroup> = Vec::new();

    for day in days {
        let week = crate::SolarWeek::from_ymd(day.solar().year(), day.solar().month(), day.solar().day(), week_start);
        let week_start_day = week.first_day();
        let week_end_day = week_start_day.next_day(6);

        if current_start == Some(week_start_day) {
            current_days.push(day);
            current_end = Some(week_end_day);
            continue;
        }

        if let (Some(start), Some(end)) = (current_start, current_end) {
            groups.push(EventWeekGroup::new(start, end, std::mem::take(&mut current_days)));
        }

        current_start = Some(week_start_day);
        current_end = Some(week_end_day);
        current_days.push(day);
    }

    if let (Some(start), Some(end)) = (current_start, current_end) {
        groups.push(EventWeekGroup::new(start, end, current_days));
    }

    groups
}

pub fn scan_event_weeks_in_range(start: Solar, end: Solar, week_start: i32) -> Vec<EventWeekGroup> {
    group_event_days_by_week(scan_event_days_in_range(start, end), week_start)
}

pub fn scan_event_weeks_in_range_filtered(
    start: Solar,
    end: Solar,
    week_start: i32,
    query: &EventQuery<'_>,
) -> Vec<EventWeekGroup> {
    group_event_days_by_week(scan_event_days_in_range_filtered(start, end, query), week_start)
}
