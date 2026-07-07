//! 塞琉古纪元（Seleucid Era / Anno Graecorum）。

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

/// 西叙基督徒固定年界版本的塞琉古纪元：
/// - 1-9 月：公历年 + 311
/// - 10-12 月：公历年 + 312
pub const EARLY_YEAR_OFFSET: i32 = 311;
pub const LATE_YEAR_OFFSET: i32 = 312;

fn seleucid_year_from_solar(solar: Solar) -> i32 {
    if solar.month() >= 10 { solar.year() + LATE_YEAR_OFFSET } else { solar.year() + EARLY_YEAR_OFFSET }
}

fn start_solar_year(seleucid_year: i32) -> i32 {
    seleucid_year - LATE_YEAR_OFFSET
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeleucidYear {
    year: i32,
}

impl SeleucidYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 312 {
            return Err(LunarError::Parse(format!("illegal seleucid year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() < 1 {
            return Err(LunarError::Parse(format!("unsupported seleucid date: {}", solar.to_ymd())));
        }
        Self::from_year(seleucid_year_from_solar(solar))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn first_month(&self) -> SeleucidMonth {
        SeleucidMonth::from_ym(self.year, 10).unwrap()
    }

    pub fn last_month(&self) -> SeleucidMonth {
        SeleucidMonth::from_ym(self.year, 9).unwrap()
    }

    pub fn months(&self) -> Vec<SeleucidMonth> {
        let mut months = Vec::with_capacity(12);
        for month in 10..=12 {
            months.push(SeleucidMonth::from_ym(self.year, month).unwrap());
        }
        for month in 1..=9 {
            months.push(SeleucidMonth::from_ym(self.year, month).unwrap());
        }
        months
    }

    pub fn day_count(&self) -> i32 {
        self.months().iter().map(SeleucidMonth::day_count).sum()
    }

    pub fn first_day(&self) -> Seleucid {
        Seleucid::from_ymd(self.year, 10, 1).unwrap()
    }

    pub fn last_day(&self) -> Seleucid {
        let last_day = solar_util::days_of_month(start_solar_year(self.year) + 1, 9);
        Seleucid::from_ymd(self.year, 9, last_day).unwrap()
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

    pub fn contains_seleucid(&self, seleucid: Seleucid) -> bool {
        seleucid.year() == self.year
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

impl CalendarSpan for SeleucidYear {
    fn first_solar_day(&self) -> Solar {
        SeleucidYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        SeleucidYear::last_solar_day(self)
    }
}

impl fmt::Display for SeleucidYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "塞琉古{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeleucidMonth {
    year: i32,
    month: i32,
}

impl SeleucidMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        SeleucidYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal seleucid month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Seleucid::from_solar(solar)?;
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn solar_year(&self) -> i32 {
        if self.month >= 10 { start_solar_year(self.year) } else { start_solar_year(self.year) + 1 }
    }

    pub fn seleucid_year(&self) -> SeleucidYear {
        SeleucidYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> Seleucid {
        Seleucid::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Seleucid {
        Seleucid::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Seleucid> {
        (1..=self.day_count()).map(|day| Seleucid::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.solar_year(), self.month, months);
        Self::from_solar(Solar::from_ymd(year, month, 1)?)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_seleucid(&self, seleucid: Seleucid) -> bool {
        seleucid.year() == self.year && seleucid.month() == self.month
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

impl CalendarSpan for SeleucidMonth {
    fn first_solar_day(&self) -> Solar {
        SeleucidMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        SeleucidMonth::last_solar_day(self)
    }
}

impl fmt::Display for SeleucidMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.seleucid_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Seleucid {
    year: i32,
    month: i32,
    day: i32,
}

impl Seleucid {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = SeleucidMonth::from_ym(year, month)?;
        Solar::from_ymd(month_obj.solar_year(), month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() < 1 {
            return Err(LunarError::Parse(format!("unsupported seleucid date: {}", solar.to_ymd())));
        }
        Ok(Self { year: seleucid_year_from_solar(solar), month: solar.month(), day: solar.day() })
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
        if self.month >= 10 { start_solar_year(self.year) } else { start_solar_year(self.year) + 1 }
    }

    pub fn seleucid_year(&self) -> SeleucidYear {
        SeleucidYear::from_year(self.year).unwrap()
    }

    pub fn seleucid_month(&self) -> SeleucidMonth {
        SeleucidMonth::from_ym(self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_seleucid_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_seleucid_full(self)
    }
}

impl CalendarDay for Seleucid {
    fn solar(&self) -> Solar {
        Seleucid::solar(self)
    }
}

impl fmt::Display for Seleucid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "塞琉古{}年{}月{}日", self.year, self.month, self.day)
    }
}
