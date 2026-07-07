//! 科普特历（Coptic calendar）。

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::{LunarError, Solar};

const ANCHOR_YEAR: i32 = 1741;
const ANCHOR_DAY: i32 = 1;
const ANCHOR_SOLAR_YEAR: i32 = 2024;
const ANCHOR_SOLAR_MONTH: i32 = 9;
const ANCHOR_SOLAR_DAY: i32 = 11;
const CYCLE_DAYS: i32 = 1461;

fn is_leap_year(year: i32) -> bool {
    year.rem_euclid(4) == 3
}

fn days_in_year(year: i32) -> i32 {
    if is_leap_year(year) { 366 } else { 365 }
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1..=12 => 30,
        13 => {
            if is_leap_year(year) {
                6
            } else {
                5
            }
        }
        _ => 0,
    }
}

fn anchor_solar() -> Solar {
    Solar::from_ymd(ANCHOR_SOLAR_YEAR, ANCHOR_SOLAR_MONTH, ANCHOR_SOLAR_DAY).unwrap()
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CopticYear {
    year: i32,
}

impl CopticYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal coptic year: {year}")));
        }
        Ok(Self { year })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn is_leap(&self) -> bool {
        is_leap_year(self.year)
    }

    pub fn day_count(&self) -> i32 {
        days_in_year(self.year)
    }

    pub fn first_month(&self) -> CopticMonth {
        CopticMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> CopticMonth {
        CopticMonth::from_ym(self.year, 13).unwrap()
    }

    pub fn months(&self) -> Vec<CopticMonth> {
        (1..=13).map(|month| CopticMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Coptic {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Coptic {
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

    pub fn contains_coptic(&self, coptic: Coptic) -> bool {
        coptic.year() == self.year
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

impl CalendarSpan for CopticYear {
    fn first_solar_day(&self) -> Solar {
        CopticYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        CopticYear::last_solar_day(self)
    }
}

impl fmt::Display for CopticYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "科普特{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CopticMonth {
    year: i32,
    month: i32,
}

impl CopticMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        CopticYear::from_year(year)?;
        if !(1..=13).contains(&month) {
            return Err(LunarError::Parse(format!("illegal coptic month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn coptic_year(&self) -> CopticYear {
        CopticYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        days_in_month(self.year, self.month)
    }

    pub fn first_day(&self) -> Coptic {
        Coptic::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Coptic {
        Coptic::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Coptic> {
        (1..=self.day_count()).map(|day| Coptic::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let total = self.year * 13 + (self.month - 1) + months;
        let year = total.div_euclid(13);
        let month = total.rem_euclid(13) + 1;
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_coptic(&self, coptic: Coptic) -> bool {
        coptic.year() == self.year && coptic.month() == self.month
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

impl CalendarSpan for CopticMonth {
    fn first_solar_day(&self) -> Solar {
        CopticMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        CopticMonth::last_solar_day(self)
    }
}

impl fmt::Display for CopticMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.coptic_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coptic {
    year: i32,
    month: i32,
    day: i32,
}

impl Coptic {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        CopticMonth::from_ym(year, month)?;
        let max = days_in_month(year, month);
        if day < 1 || day > max {
            return Err(LunarError::Parse(format!("illegal coptic day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Self {
        let total_days = solar.subtract(&anchor_solar());
        let cycles = total_days.div_euclid(CYCLE_DAYS);
        let mut remaining = total_days.rem_euclid(CYCLE_DAYS);
        let mut year = ANCHOR_YEAR + cycles * 4;

        loop {
            let len = days_in_year(year);
            if remaining < len {
                break;
            }
            remaining -= len;
            year += 1;
        }

        let mut month = 1;
        loop {
            let len = days_in_month(year, month);
            if remaining < len {
                break;
            }
            remaining -= len;
            month += 1;
        }

        Self { year, month, day: remaining + 1 }
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

    pub fn coptic_year(&self) -> CopticYear {
        CopticYear::from_year(self.year).unwrap()
    }

    pub fn coptic_month(&self) -> CopticMonth {
        CopticMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        let mut days = 0;
        if self.year >= ANCHOR_YEAR {
            for year in ANCHOR_YEAR..self.year {
                days += days_in_year(year);
            }
        } else {
            for year in self.year..ANCHOR_YEAR {
                days -= days_in_year(year);
            }
        }
        for month in 1..self.month {
            days += days_in_month(self.year, month);
        }
        days += self.day - ANCHOR_DAY;
        anchor_solar().next_day(days)
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
        crate::i18n::locale(language).render_coptic_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_coptic_full(self)
    }
}

impl CalendarDay for Coptic {
    fn solar(&self) -> Solar {
        Coptic::solar(self)
    }
}

impl fmt::Display for Coptic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "科普特{}年{}月{}日", self.year, self.month, self.day)
    }
}
