//! 鲁米历（Rumi calendar）。

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::julian::Julian;
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::solar_util;
use crate::{LunarError, Solar};

const YEAR_OFFSET: i32 = 584;
const SUPPORTED_START: (i32, i32, i32) = (1840, 3, 13);
const GREGORIAN_REALIGN: (i32, i32, i32) = (1917, 3, 1);
const JANUARY_YEAR_START: (i32, i32, i32) = (1918, 1, 1);
const FIRST_YEAR: i32 = 1256;
const TRANSITION_YEAR: i32 = 1333;

fn pre_realign(year: i32) -> bool {
    year < TRANSITION_YEAR
}

fn post_january_year_start(year: i32) -> bool {
    year > TRANSITION_YEAR
}

fn first_month_number(year: i32) -> i32 {
    if post_january_year_start(year) { 1 } else { 3 }
}

fn last_month_number(year: i32) -> i32 {
    if pre_realign(year) { 2 } else { 12 }
}

fn rumi_year_from_solar(solar: Solar) -> i32 {
    let date = (solar.year(), solar.month(), solar.day());
    if date >= JANUARY_YEAR_START {
        solar.year() - YEAR_OFFSET
    } else if date >= GREGORIAN_REALIGN {
        TRANSITION_YEAR
    } else {
        let julian = Julian::from_solar(solar);
        if julian.month() >= 3 { julian.year() - YEAR_OFFSET } else { julian.year() - YEAR_OFFSET - 1 }
    }
}

fn month_sequence(year: i32) -> &'static [i32] {
    if post_january_year_start(year) {
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    } else if year == TRANSITION_YEAR {
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    } else {
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 1, 2]
    }
}

fn shift_month(mut year: i32, mut month: i32, mut offset: i32) -> Result<(i32, i32), LunarError> {
    RumiMonth::from_ym(year, month)?;
    while offset > 0 {
        let seq = month_sequence(year);
        let index = seq
            .iter()
            .position(|candidate| *candidate == month)
            .ok_or_else(|| LunarError::Parse(format!("illegal rumi month: {year}-{month}")))?;
        if index + 1 < seq.len() {
            month = seq[index + 1];
        } else {
            year += 1;
            month = month_sequence(year)[0];
        }
        offset -= 1;
    }
    while offset < 0 {
        let seq = month_sequence(year);
        let index = seq
            .iter()
            .position(|candidate| *candidate == month)
            .ok_or_else(|| LunarError::Parse(format!("illegal rumi month: {year}-{month}")))?;
        if index > 0 {
            month = seq[index - 1];
        } else {
            year -= 1;
            RumiYear::from_year(year)?;
            let prev = month_sequence(year);
            month = prev[prev.len() - 1];
        }
        offset += 1;
    }
    Ok((year, month))
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RumiYear {
    year: i32,
}

impl RumiYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if year < FIRST_YEAR {
            return Err(LunarError::Parse(format!("illegal rumi year: {year}")));
        }
        Ok(Self { year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < SUPPORTED_START {
            return Err(LunarError::Parse(format!("unsupported rumi date: {}", solar.to_ymd())));
        }
        Self::from_year(rumi_year_from_solar(solar))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn first_month(&self) -> RumiMonth {
        RumiMonth::from_ym(self.year, first_month_number(self.year)).unwrap()
    }

    pub fn last_month(&self) -> RumiMonth {
        RumiMonth::from_ym(self.year, last_month_number(self.year)).unwrap()
    }

    pub fn months(&self) -> Vec<RumiMonth> {
        month_sequence(self.year).iter().map(|month| RumiMonth::from_ym(self.year, *month).unwrap()).collect()
    }

    pub fn day_count(&self) -> i32 {
        self.months().iter().map(RumiMonth::day_count).sum()
    }

    pub fn first_day(&self) -> Rumi {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Rumi {
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

    pub fn contains_rumi(&self, rumi: Rumi) -> bool {
        rumi.year() == self.year
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

impl CalendarSpan for RumiYear {
    fn first_solar_day(&self) -> Solar {
        RumiYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        RumiYear::last_solar_day(self)
    }
}

impl fmt::Display for RumiYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "鲁米历{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RumiMonth {
    year: i32,
    month: i32,
}

impl RumiMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        RumiYear::from_year(year)?;
        if !month_sequence(year).contains(&month) {
            return Err(LunarError::Parse(format!("illegal rumi month: {year}-{month}")));
        }
        Ok(Self { year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Rumi::from_solar(solar)?;
        Self::from_ym(date.year(), date.month())
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn solar_year(&self) -> i32 {
        if post_january_year_start(self.year) {
            self.year + YEAR_OFFSET
        } else if self.year == TRANSITION_YEAR {
            1917
        } else if self.month >= 3 {
            self.year + YEAR_OFFSET
        } else {
            self.year + YEAR_OFFSET + 1
        }
    }

    pub fn rumi_year(&self) -> RumiYear {
        RumiYear::from_year(self.year).unwrap()
    }

    pub fn day_count(&self) -> i32 {
        if self.year == 1332 && self.month == 2 {
            15
        } else if self.year == TRANSITION_YEAR {
            solar_util::days_of_month(1917, self.month)
        } else if pre_realign(self.year) {
            let julian_year = if self.month >= 3 { self.year + YEAR_OFFSET } else { self.year + YEAR_OFFSET + 1 };
            crate::julian::JulianMonth::from_ym(julian_year, self.month).unwrap().day_count()
        } else {
            solar_util::days_of_month(self.year + YEAR_OFFSET, self.month)
        }
    }

    pub fn first_day(&self) -> Rumi {
        Rumi::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_day(&self) -> Rumi {
        Rumi::from_ymd(self.year, self.month, self.day_count()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Rumi> {
        (1..=self.day_count()).map(|day| Rumi::from_ymd(self.year, self.month, day).unwrap()).collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = shift_month(self.year, self.month, months)?;
        Self::from_ym(year, month)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_rumi(&self, rumi: Rumi) -> bool {
        rumi.year() == self.year && rumi.month() == self.month
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

impl CalendarSpan for RumiMonth {
    fn first_solar_day(&self) -> Solar {
        RumiMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        RumiMonth::last_solar_day(self)
    }
}

impl fmt::Display for RumiMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.rumi_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rumi {
    year: i32,
    month: i32,
    day: i32,
}

impl Rumi {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = RumiMonth::from_ym(year, month)?;
        if day < 1 || day > month_obj.day_count() {
            return Err(LunarError::Parse(format!("illegal rumi day: {year}-{month}-{day}")));
        }
        Ok(Self { year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        if (solar.year(), solar.month(), solar.day()) < SUPPORTED_START {
            return Err(LunarError::Parse(format!("unsupported rumi date: {}", solar.to_ymd())));
        }
        let year = rumi_year_from_solar(solar);
        if (solar.year(), solar.month(), solar.day()) >= GREGORIAN_REALIGN {
            return Ok(Self { year, month: solar.month(), day: solar.day() });
        }
        let julian = Julian::from_solar(solar);
        Ok(Self { year, month: julian.month(), day: julian.day() })
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
        self.rumi_month().solar_year()
    }

    pub fn rumi_year(&self) -> RumiYear {
        RumiYear::from_year(self.year).unwrap()
    }

    pub fn rumi_month(&self) -> RumiMonth {
        RumiMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn solar(&self) -> Solar {
        if self.year >= 1334 {
            Solar::from_ymd(self.year + YEAR_OFFSET, self.month, self.day).unwrap()
        } else if self.year == TRANSITION_YEAR {
            Solar::from_ymd(1917, self.month, self.day).unwrap()
        } else {
            let julian_year = if self.month >= 3 { self.year + YEAR_OFFSET } else { self.year + YEAR_OFFSET + 1 };
            Julian::from_ymd(julian_year, self.month, self.day).unwrap().solar()
        }
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
        crate::i18n::locale(language).render_rumi_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_rumi_full(self)
    }
}

impl CalendarDay for Rumi {
    fn solar(&self) -> Solar {
        Rumi::solar(self)
    }
}

impl fmt::Display for Rumi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "鲁米历{}年{}月{}日", self.year, self.month, self.day)
    }
}
