//! 纳纳克沙希历（Nanakshahi calendar）。

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

pub const EARLY_YEAR_OFFSET: i32 = 1469;
pub const LATE_YEAR_OFFSET: i32 = 1468;
const FIRST_SUPPORTED_SOLAR: (i32, i32, i32) = (1469, 3, 14);

fn nanakshahi_year_from_solar(solar: Solar) -> i32 {
    if (solar.month(), solar.day()) >= (3, 14) {
        solar.year() - LATE_YEAR_OFFSET
    } else {
        solar.year() - EARLY_YEAR_OFFSET
    }
}

fn start_solar_year(nanakshahi_year: i32) -> i32 {
    nanakshahi_year + LATE_YEAR_OFFSET
}

fn year_start_solar(nanakshahi_year: i32) -> Solar {
    Solar::from_ymd(start_solar_year(nanakshahi_year), 3, 14).unwrap()
}

fn month_lengths(nanakshahi_year: i32) -> [i32; 12] {
    let next_solar_year = start_solar_year(nanakshahi_year) + 1;
    [31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 30, if solar_util::is_leap_year(next_solar_year) { 31 } else { 30 }]
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NanakshahiYear {
    year: i32,
}

impl NanakshahiYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal nanakshahi year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Nanakshahi::from_solar(solar)?;
        Self::from_year(date.year())
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

    pub fn is_leap(&self) -> bool {
        month_lengths(self.year)[11] == 31
    }

    pub fn first_month(&self) -> NanakshahiMonth {
        NanakshahiMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> NanakshahiMonth {
        NanakshahiMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<NanakshahiMonth> {
        (1..=12).map(|month| NanakshahiMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Nanakshahi {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Nanakshahi {
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

    pub fn contains_nanakshahi(&self, nanakshahi: Nanakshahi) -> bool {
        nanakshahi.year() == self.year
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

impl CalendarSpan for NanakshahiYear {
    fn first_solar_day(&self) -> Solar {
        NanakshahiYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        NanakshahiYear::last_solar_day(self)
    }
}

impl fmt::Display for NanakshahiYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "纳纳克沙希{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NanakshahiMonth {
    year: i32,
    month: i32,
}

impl NanakshahiMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        NanakshahiYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal nanakshahi month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Nanakshahi::from_solar(solar)?;
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

    pub fn nanakshahi_year(&self) -> NanakshahiYear {
        NanakshahiYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        month_lengths(self.year)[(self.month - 1) as usize]
    }

    pub fn first_day(&self) -> Nanakshahi {
        Nanakshahi::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Nanakshahi {
        Nanakshahi::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Nanakshahi> {
        (1..=self.day_count()).map(|day| Nanakshahi::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.year, self.month, months);
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_nanakshahi(&self, nanakshahi: Nanakshahi) -> bool {
        nanakshahi.year() == self.year && nanakshahi.month() == self.month
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

impl CalendarSpan for NanakshahiMonth {
    fn first_solar_day(&self) -> Solar {
        NanakshahiMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        NanakshahiMonth::last_solar_day(self)
    }
}

impl fmt::Display for NanakshahiMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.nanakshahi_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nanakshahi {
    year: i32,
    month: i32,
    day: i32,
}

impl Nanakshahi {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = NanakshahiMonth::from_ym(year, month)?;
        if day < 1 || day > month_obj.day_count() {
            return Err(LunarError::Parse(format!("illegal nanakshahi day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < FIRST_SUPPORTED_SOLAR {
            return Err(LunarError::Parse(format!("unsupported nanakshahi date: {}", solar.to_ymd())));
        }

        let current_year_start = Solar::from_ymd(solar.year(), 3, 14).unwrap();
        let (nanakshahi_year, start_year) = if solar.is_before(&current_year_start) {
            (nanakshahi_year_from_solar(solar), solar.year() - 1)
        } else {
            (nanakshahi_year_from_solar(solar), solar.year())
        };

        let start = Solar::from_ymd(start_year, 3, 14).unwrap();
        let mut day_offset = solar.subtract(&start);
        let lengths = month_lengths(nanakshahi_year);
        let mut month = 1;
        for len in lengths {
            if day_offset < len {
                return Ok(Self { year: nanakshahi_year, month, day: day_offset + 1 });
            }
            day_offset -= len;
            month += 1;
        }

        Err(LunarError::Parse(format!("unsupported nanakshahi date: {}", solar.to_ymd())))
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

    pub fn nanakshahi_year(&self) -> NanakshahiYear {
        NanakshahiYear::from_year(self.year).unwrap()
    }

    pub fn nanakshahi_month(&self) -> NanakshahiMonth {
        NanakshahiMonth::from_ym(self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_nanakshahi_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_nanakshahi_full(self)
    }
}

impl CalendarDay for Nanakshahi {
    fn solar(&self) -> Solar {
        Nanakshahi::solar(self)
    }
}

impl fmt::Display for Nanakshahi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "纳纳克沙希{}年{}月{}日", self.year, self.month, self.day)
    }
}
