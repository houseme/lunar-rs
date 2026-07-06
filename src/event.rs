//! Unified calendar events.
//!
//! This is the minimal Phase 3 event model that aggregates festivals, holidays
//! and JieQi into a single typed read-only API.

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

use crate::Solar;

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
    is_primary: bool,
    tags: Vec<String>,
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
            detail,
            priority,
            source_id,
            is_observed,
            is_primary,
            tags,
        }
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

    pub const fn is_primary(&self) -> bool {
        self.is_primary
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|value| value == tag)
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

pub const fn default_primary_for_kind(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::JieQi
            | EventKind::Holiday
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
        EventKind::JieQi => "jieqi",
    }
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

#[derive(Clone, Debug, Default)]
struct EventIndex {
    events: Vec<Event>,
    by_calendar_kind: HashMap<CalendarKind, Vec<usize>>,
    by_source: HashMap<EventSource, Vec<usize>>,
    by_kind: HashMap<EventKind, Vec<usize>>,
    by_is_primary: HashMap<bool, Vec<usize>>,
    by_tag: HashMap<String, Vec<usize>>,
}

impl EventIndex {
    fn new(mut events: Vec<Event>) -> Self {
        dedup_events(&mut events);

        let mut index = Self {
            events,
            by_calendar_kind: HashMap::new(),
            by_source: HashMap::new(),
            by_kind: HashMap::new(),
            by_is_primary: HashMap::new(),
            by_tag: HashMap::new(),
        };

        for (position, event) in index.events.iter().enumerate() {
            index.by_calendar_kind.entry(*event.calendar_kind()).or_default().push(position);
            index.by_source.entry(*event.source()).or_default().push(position);
            index.by_kind.entry(*event.kind()).or_default().push(position);
            index.by_is_primary.entry(event.is_primary()).or_default().push(position);
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
            query.kind.and_then(|value| self.by_kind.get(&value)),
            query.is_primary.and_then(|value| self.by_is_primary.get(&value)),
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

pub(crate) fn clear_event_index_cache() {
    EVENT_INDEX_CACHE.write().unwrap().clear();
}

fn build_all_events_for_day(solar: Solar) -> Vec<Event> {
    let mut events = solar.events();
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventQuery<'a> {
    pub calendar_kind: Option<CalendarKind>,
    pub source: Option<EventSource>,
    pub kind: Option<EventKind>,
    pub is_primary: Option<bool>,
    pub name_contains: Option<&'a str>,
    pub detail_contains: Option<&'a str>,
    pub has_tag: Option<&'a str>,
}

impl<'a> EventQuery<'a> {
    pub const fn new() -> Self {
        Self {
            calendar_kind: None,
            source: None,
            kind: None,
            is_primary: None,
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

    pub const fn with_kind(mut self, kind: EventKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub const fn with_is_primary(mut self, is_primary: bool) -> Self {
        self.is_primary = Some(is_primary);
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
        if let Some(kind) = &self.kind
            && event.kind() != kind
        {
            return false;
        }
        if let Some(is_primary) = self.is_primary
            && event.is_primary() != is_primary
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
