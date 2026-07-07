//! 西班牙纪元（Hispanic Era / Spanish era）。

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::solar::next_ym;
use crate::solar_util;
use crate::{LunarError, Solar};

/// Hispanic Era 年 = 公历年份 + 38。
pub const YEAR_OFFSET: i32 = 38;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HispanicEraYear {
    year: i32,
}

impl HispanicEraYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < YEAR_OFFSET + 1 {
            return Err(LunarError::Parse(format!("illegal hispanic era year: {year}")));
        }
        Ok(Self { year })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year - YEAR_OFFSET
    }

    pub fn is_leap(&self) -> bool {
        solar_util::is_leap_year(self.solar_year())
    }

    pub fn day_count(&self) -> i32 {
        if self.is_leap() { 366 } else { 365 }
    }

    pub fn first_month(&self) -> HispanicEraMonth {
        HispanicEraMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> HispanicEraMonth {
        HispanicEraMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<HispanicEraMonth> {
        (1..=12).map(|month| HispanicEraMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> HispanicEra {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> HispanicEra {
        self.last_month().last_day()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn next(&self, years: i32) -> Result<Self, LunarError> {
        Self::from_year(self.year + years)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_hispanic_era(&self, era: HispanicEra) -> bool {
        era.year() == self.year
    }

    pub fn events(&self) -> Vec<Event> {
        span_events(self)
    }

    pub fn all_events(&self) -> Vec<Event> {
        span_all_events(self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events(self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        span_events_until(self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events_until(self, end, query)
    }
}

impl CalendarSpan for HispanicEraYear {
    fn first_solar_day(&self) -> Solar {
        HispanicEraYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        HispanicEraYear::last_solar_day(self)
    }
}

impl fmt::Display for HispanicEraYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "西班牙纪元{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HispanicEraMonth {
    year: i32,
    month: i32,
}

impl HispanicEraMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        HispanicEraYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::InvalidSolar {
                year: year - YEAR_OFFSET,
                month,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0,
            });
        }
        Ok(Self { year, month })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year - YEAR_OFFSET
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn hispanic_era_year(&self) -> HispanicEraYear {
        HispanicEraYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> HispanicEra {
        HispanicEra::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> HispanicEra {
        HispanicEra::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<HispanicEra> {
        (1..=self.day_count()).map(|day| HispanicEra::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.year, self.month, months);
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_hispanic_era(&self, era: HispanicEra) -> bool {
        era.year() == self.year && era.month() == self.month
    }

    pub fn events(&self) -> Vec<Event> {
        span_events(self)
    }

    pub fn all_events(&self) -> Vec<Event> {
        span_all_events(self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events(self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        span_events_until(self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events_until(self, end, query)
    }
}

impl CalendarSpan for HispanicEraMonth {
    fn first_solar_day(&self) -> Solar {
        HispanicEraMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        HispanicEraMonth::last_solar_day(self)
    }
}

impl fmt::Display for HispanicEraMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.hispanic_era_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HispanicEra {
    year: i32,
    month: i32,
    day: i32,
}

impl HispanicEra {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        HispanicEraMonth::from_ym(year, month)?;
        Solar::from_ymd(year - YEAR_OFFSET, month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() < 1 {
            return Err(LunarError::Parse(format!("unsupported hispanic era date: {}", solar.to_ymd())));
        }
        Ok(Self { year: solar.year() + YEAR_OFFSET, month: solar.month(), day: solar.day() })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year - YEAR_OFFSET
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub fn hispanic_era_year(&self) -> HispanicEraYear {
        HispanicEraYear::from_year(self.year).unwrap()
    }

    pub fn hispanic_era_month(&self) -> HispanicEraMonth {
        HispanicEraMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        Solar::from_ymd(self.solar_year(), self.month, self.day).unwrap()
    }

    pub fn next(&self, days: i32) -> Result<Self, LunarError> {
        Self::from_solar(self.solar().next_day(days))
    }

    pub fn subtract(&self, other: Self) -> i32 {
        self.solar().subtract(&other.solar())
    }

    pub fn is_before(&self, other: Self) -> bool {
        self.solar().is_before(&other.solar())
    }

    pub fn is_after(&self, other: Self) -> bool {
        self.solar().is_after(&other.solar())
    }

    pub fn events(&self) -> Vec<Event> {
        point_events(self)
    }

    pub fn all_events(&self) -> Vec<Event> {
        point_all_events(self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        point_find_events(self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        point_events_until(self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        point_find_events_until(self, end, query)
    }

    pub fn to_string_cn(&self) -> String {
        self.to_string()
    }

    #[cfg(feature = "i18n")]
    pub fn to_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_hispanic_era_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_hispanic_era_full(self)
    }
}

impl CalendarDay for HispanicEra {
    fn solar(&self) -> Solar {
        HispanicEra::solar(self)
    }
}

impl fmt::Display for HispanicEra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "西班牙纪元{}年{}月{}日", self.year, self.month, self.day)
    }
}
