//! 拜占庭纪年（Byzantine calendar）。

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

/// 拜占庭纪年以 9 月 1 日为新年：
/// - 1-8 月：公历年 + 5508
/// - 9-12 月：公历年 + 5509
pub const EARLY_YEAR_OFFSET: i32 = 5508;
pub const LATE_YEAR_OFFSET: i32 = 5509;

fn byzantine_year_from_solar(solar: Solar) -> i32 {
    if solar.month() >= 9 { solar.year() + LATE_YEAR_OFFSET } else { solar.year() + EARLY_YEAR_OFFSET }
}

fn start_solar_year(byzantine_year: i32) -> i32 {
    byzantine_year - LATE_YEAR_OFFSET
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ByzantineYear {
    year: i32,
}

impl ByzantineYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 5509 {
            return Err(LunarError::Parse(format!("illegal byzantine year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        Self::from_year(byzantine_year_from_solar(solar))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn first_month(&self) -> ByzantineMonth {
        ByzantineMonth::from_ym(self.year, 9).unwrap()
    }

    pub fn last_month(&self) -> ByzantineMonth {
        ByzantineMonth::from_ym(self.year, 8).unwrap()
    }

    pub fn months(&self) -> Vec<ByzantineMonth> {
        let mut months = Vec::with_capacity(12);
        for month in 9..=12 {
            months.push(ByzantineMonth::from_ym(self.year, month).unwrap());
        }
        for month in 1..=8 {
            months.push(ByzantineMonth::from_ym(self.year, month).unwrap());
        }
        months
    }

    pub fn day_count(&self) -> i32 {
        self.months().iter().map(ByzantineMonth::day_count).sum()
    }

    pub fn first_day(&self) -> Byzantine {
        Byzantine::from_ymd(self.year, 9, 1).unwrap()
    }

    pub fn last_day(&self) -> Byzantine {
        Byzantine::from_ymd(self.year, 8, 31).unwrap()
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

    pub fn contains_byzantine(&self, byzantine: Byzantine) -> bool {
        byzantine.year() == self.year
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

impl CalendarSpan for ByzantineYear {
    fn first_solar_day(&self) -> Solar {
        ByzantineYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        ByzantineYear::last_solar_day(self)
    }
}

impl fmt::Display for ByzantineYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "拜占庭{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ByzantineMonth {
    year: i32,
    month: i32,
}

impl ByzantineMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        ByzantineYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal byzantine month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Byzantine::from_solar(solar);
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn solar_year(&self) -> i32 {
        if self.month >= 9 { start_solar_year(self.year) } else { start_solar_year(self.year) + 1 }
    }

    pub fn byzantine_year(&self) -> ByzantineYear {
        ByzantineYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> Byzantine {
        Byzantine::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Byzantine {
        Byzantine::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Byzantine> {
        (1..=self.day_count()).map(|day| Byzantine::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.solar_year(), self.month, months);
        Self::from_solar(Solar::from_ymd(year, month, 1)?)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_byzantine(&self, byzantine: Byzantine) -> bool {
        byzantine.year() == self.year && byzantine.month() == self.month
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

impl CalendarSpan for ByzantineMonth {
    fn first_solar_day(&self) -> Solar {
        ByzantineMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        ByzantineMonth::last_solar_day(self)
    }
}

impl fmt::Display for ByzantineMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.byzantine_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Byzantine {
    year: i32,
    month: i32,
    day: i32,
}

impl Byzantine {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = ByzantineMonth::from_ym(year, month)?;
        Solar::from_ymd(month_obj.solar_year(), month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Self {
        Self { year: byzantine_year_from_solar(solar), month: solar.month(), day: solar.day() }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub fn solar_year(&self) -> i32 {
        if self.month >= 9 { start_solar_year(self.year) } else { start_solar_year(self.year) + 1 }
    }

    pub fn byzantine_year(&self) -> ByzantineYear {
        ByzantineYear::from_year(self.year).unwrap()
    }

    pub fn byzantine_month(&self) -> ByzantineMonth {
        ByzantineMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        Solar::from_ymd(self.solar_year(), self.month, self.day).unwrap()
    }

    pub fn next(&self, days: i32) -> Self {
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
        crate::i18n::locale(language).render_byzantine_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_byzantine_full(self)
    }
}

impl CalendarDay for Byzantine {
    fn solar(&self) -> Solar {
        Byzantine::solar(self)
    }
}

impl fmt::Display for Byzantine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "拜占庭{}年{}月{}日", self.year, self.month, self.day)
    }
}
