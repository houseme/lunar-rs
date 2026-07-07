//! 儒略历（Julian calendar）。

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::solar::next_ym;
use crate::{LunarError, Solar};

fn is_leap_year(year: i32) -> bool {
    year.rem_euclid(4) == 0
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn julian_day_number(year: i32, month: i32, day: i32) -> f64 {
    let mut y = year;
    let mut m = month;
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    ((365.25 * f64::from(y + 4716)) as i64) as f64 + ((30.6001 * f64::from(m + 1)) as i64) as f64 + f64::from(day)
        - 1524.5
}

fn from_julian_day(jd: f64) -> (i32, i32, i32) {
    let a = (jd + 0.5).floor() as i64;
    let b = a + 1524;
    let c = (((b as f64) - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = (((b - d) as f64) / 30.6001).floor() as i64;
    let day = b - d - (30.6001 * e as f64).floor() as i64;
    let month = if e < 14 { e - 1 } else { e - 13 };
    let year = if month > 2 { c - 4716 } else { c - 4715 };
    (year as i32, month as i32, day as i32)
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JulianYear {
    year: i32,
}

impl JulianYear {
    pub const fn from_year(year: i32) -> Self {
        Self { year }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn is_leap(&self) -> bool {
        is_leap_year(self.year)
    }

    pub fn day_count(&self) -> i32 {
        if self.is_leap() { 366 } else { 365 }
    }

    pub fn first_month(&self) -> JulianMonth {
        JulianMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> JulianMonth {
        JulianMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<JulianMonth> {
        (1..=12).map(|month| JulianMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Julian {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Julian {
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

    pub fn contains_julian(&self, julian: Julian) -> bool {
        julian.year() == self.year
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

impl CalendarSpan for JulianYear {
    fn first_solar_day(&self) -> Solar {
        JulianYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        JulianYear::last_solar_day(self)
    }
}

impl fmt::Display for JulianYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "儒略历{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JulianMonth {
    year: i32,
    month: i32,
}

impl JulianMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal julian month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn julian_year(&self) -> JulianYear {
        JulianYear::from_year(self.year)
    }

    pub fn day_count(&self) -> i32 {
        days_in_month(self.year, self.month)
    }

    pub fn first_day(&self) -> Julian {
        Julian::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Julian {
        Julian::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Julian> {
        (1..=self.day_count()).map(|day| Julian::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Self {
        let (year, month) = next_ym(self.year, self.month, months);
        Self { year, month }
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_julian(&self, julian: Julian) -> bool {
        julian.year() == self.year && julian.month() == self.month
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

impl CalendarSpan for JulianMonth {
    fn first_solar_day(&self) -> Solar {
        JulianMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        JulianMonth::last_solar_day(self)
    }
}

impl fmt::Display for JulianMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.julian_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Julian {
    year: i32,
    month: i32,
    day: i32,
}

impl Julian {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        JulianMonth::from_ym(year, month)?;
        let max = days_in_month(year, month);
        if day < 1 || day > max {
            return Err(LunarError::Parse(format!("illegal julian day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Self {
        let (year, month, day) = from_julian_day(solar.julian_day());
        Self { year, month, day }
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

    pub fn julian_year(&self) -> JulianYear {
        JulianYear::from_year(self.year)
    }

    pub fn julian_month(&self) -> JulianMonth {
        JulianMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        Solar::from_julian_day(julian_day_number(self.year, self.month, self.day))
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
        crate::i18n::locale(language).render_julian_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_julian_full(self)
    }
}

impl CalendarDay for Julian {
    fn solar(&self) -> Solar {
        Julian::solar(self)
    }
}

impl fmt::Display for Julian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "儒略历{}年{}月{}日", self.year, self.month, self.day)
    }
}
