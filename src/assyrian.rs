//! 亚述纪年（Assyrian calendar）。

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

/// 亚述纪年以 4 月 1 日为新年：
/// - 1-3 月：公历年 + 4749
/// - 4-12 月：公历年 + 4750
pub const EARLY_YEAR_OFFSET: i32 = 4749;
pub const LATE_YEAR_OFFSET: i32 = 4750;
const FIRST_SUPPORTED_YEAR: i32 = EARLY_YEAR_OFFSET + 1;

fn assyrian_year_from_solar(solar: Solar) -> i32 {
    if solar.month() >= 4 { solar.year() + LATE_YEAR_OFFSET } else { solar.year() + EARLY_YEAR_OFFSET }
}

fn start_solar_year(assyrian_year: i32) -> i32 {
    assyrian_year - LATE_YEAR_OFFSET
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssyrianYear {
    year: i32,
}

impl AssyrianYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < FIRST_SUPPORTED_YEAR {
            return Err(LunarError::Parse(format!("illegal assyrian year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() < 1 {
            return Err(LunarError::Parse(format!("unsupported assyrian calendar date: {}", solar.to_ymd())));
        }
        Self::from_year(assyrian_year_from_solar(solar))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    fn first_month_number(&self) -> i32 {
        if self.year == FIRST_SUPPORTED_YEAR { 1 } else { 4 }
    }

    pub fn first_month(&self) -> AssyrianMonth {
        AssyrianMonth::from_ym(self.year, self.first_month_number()).unwrap()
    }

    pub fn last_month(&self) -> AssyrianMonth {
        AssyrianMonth::from_ym(self.year, 3).unwrap()
    }

    pub fn months(&self) -> Vec<AssyrianMonth> {
        if self.year == FIRST_SUPPORTED_YEAR {
            (1..=3).map(|month| AssyrianMonth::from_ym(self.year, month).unwrap()).collect()
        } else {
            let mut months = Vec::with_capacity(12);
            for month in 4..=12 {
                months.push(AssyrianMonth::from_ym(self.year, month).unwrap());
            }
            for month in 1..=3 {
                months.push(AssyrianMonth::from_ym(self.year, month).unwrap());
            }
            months
        }
    }

    pub fn day_count(&self) -> i32 {
        self.months().iter().map(AssyrianMonth::day_count).sum()
    }

    pub fn first_day(&self) -> Assyrian {
        Assyrian::from_ymd(self.year, self.first_month_number(), 1).unwrap()
    }

    pub fn last_day(&self) -> Assyrian {
        let day = solar_util::days_of_month(start_solar_year(self.year) + 1, 3);
        Assyrian::from_ymd(self.year, 3, day).unwrap()
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

    pub fn contains_assyrian(&self, assyrian: Assyrian) -> bool {
        assyrian.year() == self.year
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

impl CalendarSpan for AssyrianYear {
    fn first_solar_day(&self) -> Solar {
        AssyrianYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        AssyrianYear::last_solar_day(self)
    }
}

impl fmt::Display for AssyrianYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "亚述{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssyrianMonth {
    year: i32,
    month: i32,
}

impl AssyrianMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        let year_obj = AssyrianYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal assyrian month: {year}-{month}")));
        }
        if year == FIRST_SUPPORTED_YEAR && month > 3 {
            return Err(LunarError::Parse(format!("illegal assyrian month: {year}-{month}")));
        }
        if year_obj.first_month_number() == 4 || year == FIRST_SUPPORTED_YEAR {
            Ok(Self { year, month })
        } else {
            Err(LunarError::Parse(format!("illegal assyrian month: {year}-{month}")))
        }
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Assyrian::from_solar(solar)?;
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn solar_year(&self) -> i32 {
        if self.year == FIRST_SUPPORTED_YEAR || self.month < 4 {
            start_solar_year(self.year) + 1
        } else {
            start_solar_year(self.year)
        }
    }

    pub fn assyrian_year(&self) -> AssyrianYear {
        AssyrianYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> Assyrian {
        Assyrian::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Assyrian {
        Assyrian::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Assyrian> {
        (1..=self.day_count()).map(|day| Assyrian::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.solar_year(), self.month, months);
        Self::from_solar(Solar::from_ymd(year, month, 1)?)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_assyrian(&self, assyrian: Assyrian) -> bool {
        assyrian.year() == self.year && assyrian.month() == self.month
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

impl CalendarSpan for AssyrianMonth {
    fn first_solar_day(&self) -> Solar {
        AssyrianMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        AssyrianMonth::last_solar_day(self)
    }
}

impl fmt::Display for AssyrianMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.assyrian_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Assyrian {
    year: i32,
    month: i32,
    day: i32,
}

impl Assyrian {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = AssyrianMonth::from_ym(year, month)?;
        Solar::from_ymd(month_obj.solar_year(), month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if solar.year() < 1 {
            return Err(LunarError::Parse(format!("unsupported assyrian calendar date: {}", solar.to_ymd())));
        }
        Ok(Self { year: assyrian_year_from_solar(solar), month: solar.month(), day: solar.day() })
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
        if self.year == FIRST_SUPPORTED_YEAR || self.month < 4 {
            start_solar_year(self.year) + 1
        } else {
            start_solar_year(self.year)
        }
    }

    pub fn assyrian_year(&self) -> AssyrianYear {
        AssyrianYear::from_year(self.year).unwrap()
    }

    pub fn assyrian_month(&self) -> AssyrianMonth {
        AssyrianMonth::from_ym(self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_assyrian_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_assyrian_full(self)
    }
}

impl CalendarDay for Assyrian {
    fn solar(&self) -> Solar {
        Assyrian::solar(self)
    }
}

impl fmt::Display for Assyrian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "亚述{}年{}月{}日", self.year, self.month, self.day)
    }
}
