//! 民国历（Republic of China calendar）。

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

/// 民国元年对应公历 1912 年。
pub const EPOCH_YEAR: i32 = 1911;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MinguoYear {
    year: i32,
}

impl MinguoYear {
    pub const fn from_year(year: i32) -> Self {
        Self { year }
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

    pub fn first_month(&self) -> MinguoMonth {
        MinguoMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> MinguoMonth {
        MinguoMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<MinguoMonth> {
        (1..=12).map(|month| MinguoMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Minguo {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Minguo {
        self.last_month().last_day()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn next(&self, years: i32) -> Self {
        Self::from_year(self.year + years)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_minguo(&self, minguo: Minguo) -> bool {
        minguo.year() == self.year
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

impl CalendarSpan for MinguoYear {
    fn first_solar_day(&self) -> Solar {
        MinguoYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        MinguoYear::last_solar_day(self)
    }
}

impl fmt::Display for MinguoYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "民国{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MinguoMonth {
    year: i32,
    month: i32,
}

impl MinguoMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
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

    pub fn minguo_year(&self) -> MinguoYear {
        MinguoYear::from_year(self.year)
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> Minguo {
        Minguo::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Minguo {
        Minguo::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Minguo> {
        (1..=self.day_count()).map(|day| Minguo::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Self {
        let (year, month) = next_ym(self.year, self.month, months);
        Self { year, month }
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_minguo(&self, minguo: Minguo) -> bool {
        minguo.year() == self.year && minguo.month() == self.month
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

impl CalendarSpan for MinguoMonth {
    fn first_solar_day(&self) -> Solar {
        MinguoMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        MinguoMonth::last_solar_day(self)
    }
}

impl fmt::Display for MinguoMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.minguo_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Minguo {
    year: i32,
    month: i32,
    day: i32,
}

impl Minguo {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let solar_year = year + EPOCH_YEAR;
        Solar::from_ymd(solar_year, month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Self {
        Self { year: solar.year() - EPOCH_YEAR, month: solar.month(), day: solar.day() }
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

    pub fn minguo_year(&self) -> MinguoYear {
        MinguoYear::from_year(self.year)
    }

    pub fn minguo_month(&self) -> MinguoMonth {
        MinguoMonth::from_ym(self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_minguo_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_minguo_full(self)
    }
}

impl CalendarDay for Minguo {
    fn solar(&self) -> Solar {
        Minguo::solar(self)
    }
}

impl fmt::Display for Minguo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "民国{}年{}月{}日", self.year, self.month, self.day)
    }
}
