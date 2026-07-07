//! 主体纪年（Juche calendar）。

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

/// 主体纪元起点：公历 1912 年 = 主体 1 年。
pub const EPOCH_YEAR: i32 = 1911;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JucheYear {
    year: i32,
}

impl JucheYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal juche year: {year}")));
        }
        Ok(Self { year })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + EPOCH_YEAR
    }

    pub fn is_leap(&self) -> bool {
        solar_util::is_leap_year(self.solar_year())
    }

    pub fn day_count(&self) -> i32 {
        if self.is_leap() { 366 } else { 365 }
    }

    pub fn first_month(&self) -> JucheMonth {
        JucheMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> JucheMonth {
        JucheMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<JucheMonth> {
        (1..=12).map(|month| JucheMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Juche {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Juche {
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

    pub fn contains_juche(&self, juche: Juche) -> bool {
        juche.year() == self.year
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

impl CalendarSpan for JucheYear {
    fn first_solar_day(&self) -> Solar {
        JucheYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        JucheYear::last_solar_day(self)
    }
}

impl fmt::Display for JucheYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "主体{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JucheMonth {
    year: i32,
    month: i32,
}

impl JucheMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        JucheYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::InvalidSolar {
                year: year + EPOCH_YEAR,
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
        self.year + EPOCH_YEAR
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn juche_year(&self) -> JucheYear {
        JucheYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> Juche {
        Juche::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Juche {
        Juche::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Juche> {
        (1..=self.day_count()).map(|day| Juche::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.year, self.month, months);
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_juche(&self, juche: Juche) -> bool {
        juche.year() == self.year && juche.month() == self.month
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

impl CalendarSpan for JucheMonth {
    fn first_solar_day(&self) -> Solar {
        JucheMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        JucheMonth::last_solar_day(self)
    }
}

impl fmt::Display for JucheMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.juche_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Juche {
    year: i32,
    month: i32,
    day: i32,
}

impl Juche {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        JucheMonth::from_ym(year, month)?;
        Solar::from_ymd(year + EPOCH_YEAR, month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() <= EPOCH_YEAR {
            return Err(LunarError::Parse(format!("unsupported juche calendar date: {}", solar.to_ymd())));
        }
        Ok(Self { year: solar.year() - EPOCH_YEAR, month: solar.month(), day: solar.day() })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + EPOCH_YEAR
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub fn juche_year(&self) -> JucheYear {
        JucheYear::from_year(self.year).unwrap()
    }

    pub fn juche_month(&self) -> JucheMonth {
        JucheMonth::from_ym(self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_juche_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_juche_full(self)
    }
}

impl CalendarDay for Juche {
    fn solar(&self) -> Solar {
        Juche::solar(self)
    }
}

impl fmt::Display for Juche {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "主体{}年{}月{}日", self.year, self.month, self.day)
    }
}
