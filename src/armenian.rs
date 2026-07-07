//! 亚美尼亚历（Armenian calendar）。

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::{LunarError, Solar};

const ANCHOR_YEAR: i32 = 1474;
const ANCHOR_DAY: i32 = 1;
const ANCHOR_SOLAR_YEAR: i32 = 2024;
const ANCHOR_SOLAR_MONTH: i32 = 7;
const ANCHOR_SOLAR_DAY: i32 = 24;
const DAYS_PER_YEAR: i32 = 365;

const fn days_in_month(month: i32) -> i32 {
    match month {
        1..=12 => 30,
        13 => 5,
        _ => 0,
    }
}

fn anchor_solar() -> Solar {
    Solar::from_ymd(ANCHOR_SOLAR_YEAR, ANCHOR_SOLAR_MONTH, ANCHOR_SOLAR_DAY).unwrap()
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArmenianYear {
    year: i32,
}

impl ArmenianYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal armenian year: {year}")));
        }
        Ok(Self { year })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn is_leap(&self) -> bool {
        false
    }

    pub const fn day_count(&self) -> i32 {
        DAYS_PER_YEAR
    }

    pub fn first_month(&self) -> ArmenianMonth {
        ArmenianMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> ArmenianMonth {
        ArmenianMonth::from_ym(self.year, 13).unwrap()
    }

    pub fn months(&self) -> Vec<ArmenianMonth> {
        (1..=13).map(|month| ArmenianMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Armenian {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Armenian {
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

    pub fn contains_armenian(&self, armenian: Armenian) -> bool {
        armenian.year() == self.year
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

impl CalendarSpan for ArmenianYear {
    fn first_solar_day(&self) -> Solar {
        ArmenianYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        ArmenianYear::last_solar_day(self)
    }
}

impl fmt::Display for ArmenianYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "亚美尼亚{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArmenianMonth {
    year: i32,
    month: i32,
}

impl ArmenianMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        ArmenianYear::from_year(year)?;
        if !(1..=13).contains(&month) {
            return Err(LunarError::Parse(format!("illegal armenian month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn armenian_year(&self) -> ArmenianYear {
        ArmenianYear::from_year(self.year).unwrap()
    }

    pub const fn day_count(&self) -> i32 {
        days_in_month(self.month)
    }

    pub fn first_day(&self) -> Armenian {
        Armenian::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Armenian {
        Armenian::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Armenian> {
        (1..=self.day_count()).map(|day| Armenian::from_ymd(self.year, self.month, day).unwrap()).collect()
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

    pub fn contains_armenian(&self, armenian: Armenian) -> bool {
        armenian.year() == self.year && armenian.month() == self.month
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

impl CalendarSpan for ArmenianMonth {
    fn first_solar_day(&self) -> Solar {
        ArmenianMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        ArmenianMonth::last_solar_day(self)
    }
}

impl fmt::Display for ArmenianMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.armenian_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Armenian {
    year: i32,
    month: i32,
    day: i32,
}

impl Armenian {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        ArmenianMonth::from_ym(year, month)?;
        let max = days_in_month(month);
        if day < 1 || day > max {
            return Err(LunarError::Parse(format!("illegal armenian day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Self {
        let total_days = solar.subtract(&anchor_solar());
        let year = ANCHOR_YEAR + total_days.div_euclid(DAYS_PER_YEAR);
        let mut remaining = total_days.rem_euclid(DAYS_PER_YEAR);

        let mut month = 1;
        loop {
            let len = days_in_month(month);
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

    pub fn armenian_year(&self) -> ArmenianYear {
        ArmenianYear::from_year(self.year).unwrap()
    }

    pub fn armenian_month(&self) -> ArmenianMonth {
        ArmenianMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        let mut days = (self.year - ANCHOR_YEAR) * DAYS_PER_YEAR;
        for month in 1..self.month {
            days += days_in_month(month);
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
        crate::i18n::locale(language).render_armenian_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_armenian_full(self)
    }
}

impl CalendarDay for Armenian {
    fn solar(&self) -> Solar {
        Armenian::solar(self)
    }
}

impl fmt::Display for Armenian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "亚美尼亚{}年{}月{}日", self.year, self.month, self.day)
    }
}
