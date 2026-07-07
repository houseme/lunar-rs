//! 威尼斯纪年（Venetian calendar / More Veneto）。

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

/// 威尼斯纪年以 3 月 1 日为新年：
/// - 1-2 月：仍算上一年
/// - 3-12 月：与公历年份一致
pub const EARLY_YEAR_OFFSET: i32 = -1;
pub const LATE_YEAR_OFFSET: i32 = 0;
const SUPPORTED_START: (i32, i32, i32) = (1, 3, 1);

fn venetian_year_from_solar(solar: Solar) -> i32 {
    if solar.month() >= 3 { solar.year() + LATE_YEAR_OFFSET } else { solar.year() + EARLY_YEAR_OFFSET }
}

fn start_solar_year(year: i32) -> i32 {
    year - LATE_YEAR_OFFSET
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VenetianYear {
    year: i32,
}

impl VenetianYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < 1 {
            return Err(LunarError::Parse(format!("illegal venetian year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < SUPPORTED_START {
            return Err(LunarError::Parse(format!("unsupported venetian date: {}", solar.to_ymd())));
        }
        Self::from_year(venetian_year_from_solar(solar))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn first_month(&self) -> VenetianMonth {
        VenetianMonth::from_ym(self.year, 3).unwrap()
    }

    pub fn last_month(&self) -> VenetianMonth {
        VenetianMonth::from_ym(self.year, 2).unwrap()
    }

    pub fn months(&self) -> Vec<VenetianMonth> {
        let mut months = Vec::with_capacity(12);
        for month in 3..=12 {
            months.push(VenetianMonth::from_ym(self.year, month).unwrap());
        }
        for month in 1..=2 {
            months.push(VenetianMonth::from_ym(self.year, month).unwrap());
        }
        months
    }

    pub fn day_count(&self) -> i32 {
        self.months().iter().map(VenetianMonth::day_count).sum()
    }

    pub fn first_day(&self) -> Venetian {
        Venetian::from_ymd(self.year, 3, 1).unwrap()
    }

    pub fn last_day(&self) -> Venetian {
        let last_day = solar_util::days_of_month(start_solar_year(self.year) + 1, 2);
        Venetian::from_ymd(self.year, 2, last_day).unwrap()
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

    pub fn contains_venetian(&self, date: Venetian) -> bool {
        date.year() == self.year
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

impl CalendarSpan for VenetianYear {
    fn first_solar_day(&self) -> Solar {
        VenetianYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        VenetianYear::last_solar_day(self)
    }
}

impl fmt::Display for VenetianYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "威尼斯{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VenetianMonth {
    year: i32,
    month: i32,
}

impl VenetianMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        VenetianYear::from_year(year)?;
        if !(1..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal venetian month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Venetian::from_solar(solar)?;
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn solar_year(&self) -> i32 {
        if self.month >= 3 { start_solar_year(self.year) } else { start_solar_year(self.year) + 1 }
    }

    pub fn venetian_year(&self) -> VenetianYear {
        VenetianYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        solar_util::days_of_month(self.solar_year(), self.month)
    }

    pub fn first_day(&self) -> Venetian {
        Venetian::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Venetian {
        Venetian::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Venetian> {
        (1..=self.day_count()).map(|day| Venetian::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.solar_year(), self.month, months);
        Self::from_solar(Solar::from_ymd(year, month, 1)?)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_venetian(&self, date: Venetian) -> bool {
        date.year() == self.year && date.month() == self.month
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

impl CalendarSpan for VenetianMonth {
    fn first_solar_day(&self) -> Solar {
        VenetianMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        VenetianMonth::last_solar_day(self)
    }
}

impl fmt::Display for VenetianMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.venetian_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Venetian {
    year: i32,
    month: i32,
    day: i32,
}

impl Venetian {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = VenetianMonth::from_ym(year, month)?;
        Solar::from_ymd(month_obj.solar_year(), month, day)?;
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < SUPPORTED_START {
            return Err(LunarError::Parse(format!("unsupported venetian date: {}", solar.to_ymd())));
        }
        Ok(Self { year: venetian_year_from_solar(solar), month: solar.month(), day: solar.day() })
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
        if self.month >= 3 { start_solar_year(self.year) } else { start_solar_year(self.year) + 1 }
    }

    pub fn venetian_year(&self) -> VenetianYear {
        VenetianYear::from_year(self.year).unwrap()
    }

    pub fn venetian_month(&self) -> VenetianMonth {
        VenetianMonth::from_ym(self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_venetian_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_venetian_full(self)
    }
}

impl CalendarDay for Venetian {
    fn solar(&self) -> Solar {
        Venetian::solar(self)
    }
}

impl fmt::Display for Venetian {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "威尼斯{}年{}月{}日", self.year, self.month, self.day)
    }
}
