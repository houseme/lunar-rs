//! 萨卡历（Saka / Indian national calendar）。

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

pub const YEAR_OFFSET: i32 = 78;
const FIRST_SUPPORTED_SOLAR_YEAR: i32 = 79;

fn start_day_in_march(solar_year: i32) -> i32 {
    if solar_util::is_leap_year(solar_year) { 21 } else { 22 }
}

fn start_of_saka_year(solar_year: i32) -> Solar {
    Solar::from_ymd(solar_year, 3, start_day_in_march(solar_year)).unwrap()
}

fn month_lengths(solar_year: i32) -> [i32; 12] {
    if solar_util::is_leap_year(solar_year) {
        [31, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 30]
    } else {
        [30, 31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 30]
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SakaYear {
    year: i32,
}

impl SakaYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal saka year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Saka::from_solar(solar)?;
        Self::from_year(date.year())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + YEAR_OFFSET
    }

    pub fn is_leap(&self) -> bool {
        solar_util::is_leap_year(self.solar_year())
    }

    pub fn day_count(&self) -> i32 {
        month_lengths(self.solar_year()).iter().sum()
    }

    pub fn first_month(&self) -> SakaMonth {
        SakaMonth::from_ym(self.year, 1).unwrap()
    }

    pub fn last_month(&self) -> SakaMonth {
        SakaMonth::from_ym(self.year, 12).unwrap()
    }

    pub fn months(&self) -> Vec<SakaMonth> {
        (1..=12).map(|month| SakaMonth::from_ym(self.year, month).unwrap()).collect()
    }

    pub fn first_day(&self) -> Saka {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Saka {
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

    pub fn contains_saka(&self, saka: Saka) -> bool {
        saka.year() == self.year
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

impl CalendarSpan for SakaYear {
    fn first_solar_day(&self) -> Solar {
        SakaYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        SakaYear::last_solar_day(self)
    }
}

impl fmt::Display for SakaYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "萨卡{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SakaMonth {
    year: i32,
    month: i32,
}

impl SakaMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        SakaYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal saka month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Saka::from_solar(solar)?;
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + YEAR_OFFSET
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn saka_year(&self) -> SakaYear {
        SakaYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        month_lengths(self.solar_year())[(self.month - 1) as usize]
    }

    pub fn first_day(&self) -> Saka {
        Saka::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Saka {
        Saka::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Saka> {
        (1..=self.day_count()).map(|day| Saka::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.year, self.month, months);
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_saka(&self, saka: Saka) -> bool {
        saka.year() == self.year && saka.month() == self.month
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

impl CalendarSpan for SakaMonth {
    fn first_solar_day(&self) -> Solar {
        SakaMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        SakaMonth::last_solar_day(self)
    }
}

impl fmt::Display for SakaMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.saka_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Saka {
    year: i32,
    month: i32,
    day: i32,
}

impl Saka {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = SakaMonth::from_ym(year, month)?;
        if day < 1 || day > month_obj.day_count() {
            return Err(LunarError::Parse(format!("illegal saka day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() < FIRST_SUPPORTED_SOLAR_YEAR
            || (solar.year() == FIRST_SUPPORTED_SOLAR_YEAR
                && (solar.month(), solar.day()) < (3, start_day_in_march(FIRST_SUPPORTED_SOLAR_YEAR)))
        {
            return Err(LunarError::Parse(format!("unsupported saka date: {}", solar.to_ymd())));
        }

        let current_year_start = start_of_saka_year(solar.year());
        let (saka_year, start) = if solar.is_before(&current_year_start) {
            let previous_solar_year = solar.year() - 1;
            (solar.year() - YEAR_OFFSET - 1, start_of_saka_year(previous_solar_year))
        } else {
            (solar.year() - YEAR_OFFSET, current_year_start)
        };

        let mut day_offset = solar.subtract(&start);
        let lengths = month_lengths(start.year());
        let mut month = 1;
        for len in lengths {
            if day_offset < len {
                return Ok(Self { year: saka_year, month, day: day_offset + 1 });
            }
            day_offset -= len;
            month += 1;
        }

        Err(LunarError::Parse(format!("unsupported saka date: {}", solar.to_ymd())))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn solar_year(&self) -> i32 {
        self.year + YEAR_OFFSET
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub fn saka_year(&self) -> SakaYear {
        SakaYear::from_year(self.year).unwrap()
    }

    pub fn saka_month(&self) -> SakaMonth {
        SakaMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        let start = start_of_saka_year(self.solar_year());
        let lengths = month_lengths(self.solar_year());
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
        crate::i18n::locale(language).render_saka_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_saka_full(self)
    }
}

impl CalendarDay for Saka {
    fn solar(&self) -> Solar {
        Saka::solar(self)
    }
}

impl fmt::Display for Saka {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "萨卡{}年{}月{}日", self.year, self.month, self.day)
    }
}
