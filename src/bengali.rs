//! 孟加拉历（Bangladeshi revised Bengali calendar）。

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

pub const EARLY_YEAR_OFFSET: i32 = 594;
pub const LATE_YEAR_OFFSET: i32 = 593;
const FIRST_SUPPORTED_SOLAR: (i32, i32, i32) = (594, 4, 14);

fn bengali_year_from_solar(solar: Solar) -> i32 {
    if (solar.month(), solar.day()) >= (4, 14) {
        solar.year() - LATE_YEAR_OFFSET
    } else {
        solar.year() - EARLY_YEAR_OFFSET
    }
}

fn start_solar_year(bengali_year: i32) -> i32 {
    bengali_year + LATE_YEAR_OFFSET
}

fn year_start_solar(bengali_year: i32) -> Solar {
    Solar::from_ymd(start_solar_year(bengali_year), 4, 14).unwrap()
}

fn month_lengths(bengali_year: i32) -> [i32; 12] {
    let next_solar_year = start_solar_year(bengali_year) + 1;
    [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, if solar_util::is_leap_year(next_solar_year) { 30 } else { 29 }, 30]
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BengaliYear {
    year: i32,
}

impl BengaliYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal bengali year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < FIRST_SUPPORTED_SOLAR {
            return Err(LunarError::Parse(format!("unsupported bengali date: {}", solar.to_ymd())));
        }
        Self::from_year(bengali_year_from_solar(solar))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + LATE_YEAR_OFFSET
    }

    pub fn day_count(&self) -> i32 {
        month_lengths(self.year).iter().sum()
    }

    pub fn first_month(&self) -> BengaliMonth {
        BengaliMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> BengaliMonth {
        BengaliMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<BengaliMonth> {
        (1..=12).map(|month| BengaliMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Bengali {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Bengali {
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

    pub fn contains_bengali(&self, bengali: Bengali) -> bool {
        bengali.year() == self.year
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

impl CalendarSpan for BengaliYear {
    fn first_solar_day(&self) -> Solar {
        BengaliYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        BengaliYear::last_solar_day(self)
    }
}

impl fmt::Display for BengaliYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "孟加拉{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BengaliMonth {
    year: i32,
    month: i32,
}

impl BengaliMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        BengaliYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal bengali month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Bengali::from_solar(solar)?;
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + LATE_YEAR_OFFSET
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn bengali_year(&self) -> BengaliYear {
        BengaliYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        month_lengths(self.year)[(self.month - 1) as usize]
    }

    pub fn first_day(&self) -> Bengali {
        Bengali::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Bengali {
        Bengali::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Bengali> {
        (1..=self.day_count()).map(|day| Bengali::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.year, self.month, months);
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_bengali(&self, bengali: Bengali) -> bool {
        bengali.year() == self.year && bengali.month() == self.month
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

impl CalendarSpan for BengaliMonth {
    fn first_solar_day(&self) -> Solar {
        BengaliMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        BengaliMonth::last_solar_day(self)
    }
}

impl fmt::Display for BengaliMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.bengali_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Bengali {
    year: i32,
    month: i32,
    day: i32,
}

impl Bengali {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = BengaliMonth::from_ym(year, month)?;
        if day < 1 || day > month_obj.day_count() {
            return Err(LunarError::Parse(format!("illegal bengali day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < FIRST_SUPPORTED_SOLAR {
            return Err(LunarError::Parse(format!("unsupported bengali date: {}", solar.to_ymd())));
        }

        let current_year_start = year_start_solar(solar.year() - LATE_YEAR_OFFSET);
        let (bengali_year, start) = if solar.is_before(&current_year_start) {
            let year = solar.year() - EARLY_YEAR_OFFSET;
            (year, year_start_solar(year))
        } else {
            let year = solar.year() - LATE_YEAR_OFFSET;
            (year, year_start_solar(year))
        };

        let mut day_offset = solar.subtract(&start);
        let lengths = month_lengths(bengali_year);
        let mut month = 1;
        for len in lengths {
            if day_offset < len {
                return Ok(Self { year: bengali_year, month, day: day_offset + 1 });
            }
            day_offset -= len;
            month += 1;
        }

        Err(LunarError::Parse(format!("unsupported bengali date: {}", solar.to_ymd())))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + LATE_YEAR_OFFSET
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub fn bengali_year(&self) -> BengaliYear {
        BengaliYear::from_year(self.year).unwrap()
    }

    pub fn bengali_month(&self) -> BengaliMonth {
        BengaliMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        let start = year_start_solar(self.year);
        let lengths = month_lengths(self.year);
        let days_before: i32 = lengths[..(self.month - 1) as usize].iter().sum();
        start.next_day(days_before + self.day - 1)
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
        crate::i18n::locale(language).render_bengali_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_bengali_full(self)
    }
}

impl CalendarDay for Bengali {
    fn solar(&self) -> Solar {
        Bengali::solar(self)
    }
}

impl fmt::Display for Bengali {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "孟加拉{}年{}月{}日", self.year, self.month, self.day)
    }
}
