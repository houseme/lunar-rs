//! Hijri / Islamic civil calendar date.
//!
//! This initial Phase 4 slice keeps the model intentionally small:
//! a validated date object plus round-trip conversion with [`crate::Solar`].

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::{LunarError, Solar};

const EPOCH_YEAR: i32 = 622;
const EPOCH_MONTH: i32 = 7;
const EPOCH_DAY: i32 = 16;
const CYCLE_DAYS: i32 = 10_631;
const MONTH_NAMES: [&str; 12] = [
    "穆哈兰姆月",
    "色法尔月",
    "赖比尔·敖外鲁月",
    "赖比尔·阿色尼月",
    "主马达·敖外鲁月",
    "主马达·阿色尼月",
    "赖哲卜月",
    "舍尔邦月",
    "赖买丹月",
    "闪瓦鲁月",
    "都尔喀尔德月",
    "都尔黑哲月",
];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HijriYear {
    year: i32,
}

impl HijriYear {
    pub const fn from_year(year: i32) -> Self {
        Self { year }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn is_leap(&self) -> bool {
        Hijri::is_leap_year_for(self.year)
    }

    pub fn day_count(&self) -> i32 {
        Hijri::days_in_year(self.year)
    }

    pub fn first_month(&self) -> HijriMonth {
        HijriMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn first_day(&self) -> Hijri {
        self.first_month().first_day()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_month(&self) -> HijriMonth {
        HijriMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn last_day(&self) -> Hijri {
        let month = self.last_month();
        Hijri::from_ymd(self.year, 12, month.day_count()).unwrap()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn months(&self) -> Vec<HijriMonth> {
        (1..=12).map(|month| HijriMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn next(&self, years: i32) -> Self {
        Self::from_year(self.year + years)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_hijri(&self, hijri: Hijri) -> bool {
        hijri.year() == self.year
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

impl CalendarSpan for HijriYear {
    fn first_solar_day(&self) -> Solar {
        HijriYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        HijriYear::last_solar_day(self)
    }
}

impl fmt::Display for HijriYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HijriMonth {
    year: i32,
    month: i32,
}

impl HijriMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        if !(1..=12).contains(&month) {
            return Err(LunarError::InvalidHijri { year, month, day: 1 });
        }
        Ok(Self { year, month })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn month_name(&self) -> &'static str {
        MONTH_NAMES[(self.month - 1) as usize]
    }

    pub fn hijri_year(&self) -> HijriYear {
        HijriYear::from_year(self.year)
    }

    pub fn day_count(&self) -> i32 {
        Hijri::days_in_month(self.year, self.month)
    }

    pub fn first_day(&self) -> Hijri {
        Hijri::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_day(&self) -> Hijri {
        Hijri::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Hijri> {
        (1..=self.day_count()).map(|day| Hijri::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Self {
        let month_index = self.year * 12 + (self.month - 1) + months;
        let year = month_index.div_euclid(12);
        let month = month_index.rem_euclid(12) + 1;
        Self { year, month }
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_hijri(&self, hijri: Hijri) -> bool {
        hijri.year() == self.year && hijri.month() == self.month
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

impl CalendarSpan for HijriMonth {
    fn first_solar_day(&self) -> Solar {
        HijriMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        HijriMonth::last_solar_day(self)
    }
}

impl fmt::Display for HijriMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.hijri_year(), self.month_name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Hijri {
    year: i32,
    month: i32,
    day: i32,
}

impl Hijri {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        if !(1..=12).contains(&month) || day < 1 {
            return Err(LunarError::InvalidHijri { year, month, day });
        }
        let max = Self::days_in_month(year, month);
        if day > max {
            return Err(LunarError::InvalidHijri { year, month, day });
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Self {
        let epoch = Solar::from_ymd(EPOCH_YEAR, EPOCH_MONTH, EPOCH_DAY).unwrap();
        let total_days = solar.subtract(&epoch);

        let cycle = total_days.div_euclid(CYCLE_DAYS);
        let mut remaining = total_days.rem_euclid(CYCLE_DAYS);
        let mut year = cycle * 30 + 1;

        loop {
            let len = Self::days_in_year(year);
            if remaining < len {
                break;
            }
            remaining -= len;
            year += 1;
        }

        let mut month = 1_i32;
        loop {
            let len = Self::days_in_month(year, month);
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

    pub fn hijri_year(&self) -> HijriYear {
        HijriYear::from_year(self.year)
    }

    pub fn hijri_month(&self) -> HijriMonth {
        HijriMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn month_name(&self) -> &'static str {
        MONTH_NAMES[(self.month - 1) as usize]
    }

    pub fn is_leap_year(&self) -> bool {
        Self::is_leap_year_for(self.year)
    }

    pub fn day_count_of_year(&self) -> i32 {
        Self::days_in_year(self.year)
    }

    pub fn day_count_of_month(&self) -> i32 {
        Self::days_in_month(self.year, self.month)
    }

    pub fn solar(&self) -> Solar {
        let epoch = Solar::from_ymd(EPOCH_YEAR, EPOCH_MONTH, EPOCH_DAY).unwrap();

        let previous_year = self.year - 1;
        let cycles = previous_year.div_euclid(30);
        let mut days = cycles * CYCLE_DAYS;
        let mut year = cycles * 30 + 1;
        while year < self.year {
            days += Self::days_in_year(year);
            year += 1;
        }

        let mut month = 1_i32;
        while month < self.month {
            days += Self::days_in_month(self.year, month);
            month += 1;
        }

        days += self.day - 1;
        epoch.next_day(days)
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

    fn is_leap_year_for(year: i32) -> bool {
        matches!((year - 1).rem_euclid(30) + 1, 2 | 5 | 7 | 10 | 13 | 16 | 18 | 21 | 24 | 26 | 29)
    }

    fn days_in_year(year: i32) -> i32 {
        if Self::is_leap_year_for(year) { 355 } else { 354 }
    }

    fn days_in_month(year: i32, month: i32) -> i32 {
        let mut days = if month % 2 == 1 { 30 } else { 29 };
        if month == 12 && Self::is_leap_year_for(year) {
            days += 1;
        }
        days
    }
}

impl CalendarDay for Hijri {
    fn solar(&self) -> Solar {
        Hijri::solar(self)
    }
}

impl fmt::Display for Hijri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年{}{}日", self.year, self.month_name(), self.day)
    }
}
