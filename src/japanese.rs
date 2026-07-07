//! 现代日本年号历（Taisho / Showa / Heisei / Reiwa）。

use std::fmt;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::solar::next_ym;
use crate::{LunarError, Solar};

const SUPPORTED_START: (i32, i32, i32) = (1912, 7, 30);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JapaneseEra {
    Taisho,
    Showa,
    Heisei,
    Reiwa,
}

impl JapaneseEra {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Taisho => "大正",
            Self::Showa => "昭和",
            Self::Heisei => "平成",
            Self::Reiwa => "令和",
        }
    }

    const fn start_ymd(&self) -> (i32, i32, i32) {
        match self {
            Self::Taisho => (1912, 7, 30),
            Self::Showa => (1926, 12, 25),
            Self::Heisei => (1989, 1, 8),
            Self::Reiwa => (2019, 5, 1),
        }
    }

    const fn end_ymd(&self) -> Option<(i32, i32, i32)> {
        match self {
            Self::Taisho => Some((1926, 12, 24)),
            Self::Showa => Some((1989, 1, 7)),
            Self::Heisei => Some((2019, 4, 30)),
            Self::Reiwa => None,
        }
    }

    fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = (solar.year(), solar.month(), solar.day());
        if date < SUPPORTED_START {
            return Err(LunarError::Parse(format!("unsupported japanese calendar date: {}", solar.to_ymd())));
        }
        if date >= Self::Reiwa.start_ymd() {
            Ok(Self::Reiwa)
        } else if date >= Self::Heisei.start_ymd() {
            Ok(Self::Heisei)
        } else if date >= Self::Showa.start_ymd() {
            Ok(Self::Showa)
        } else {
            Ok(Self::Taisho)
        }
    }

    fn max_year(&self) -> i32 {
        let start = self.start_ymd().0;
        match self.end_ymd() {
            Some((end, _, _)) => end - start + 1,
            None => 9999 - start + 1,
        }
    }
}

impl fmt::Display for JapaneseEra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JapaneseYear {
    era: JapaneseEra,
    year: i32,
}

impl JapaneseYear {
    pub fn from_era_year(era: JapaneseEra, year: i32) -> Result<Self, LunarError> {
        if year < 1 || year > era.max_year() {
            return Err(LunarError::Parse(format!("illegal japanese year: {}{}", era.name(), year)));
        }
        Ok(Self { era, year })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let era = JapaneseEra::from_solar(solar)?;
        Self::from_era_year(era, solar.year() - era.start_ymd().0 + 1)
    }

    pub const fn era(&self) -> JapaneseEra {
        self.era
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn solar_year(&self) -> i32 {
        self.era.start_ymd().0 + self.year - 1
    }

    pub fn first_month(&self) -> JapaneseMonth {
        JapaneseMonth::from_eym(self.era, self.year, self.first_month_number()).unwrap()
    }

    pub fn last_month(&self) -> JapaneseMonth {
        JapaneseMonth::from_eym(self.era, self.year, self.last_month_number()).unwrap()
    }

    pub fn months(&self) -> Vec<JapaneseMonth> {
        (self.first_month_number()..=self.last_month_number())
            .map(|month| JapaneseMonth::from_eym(self.era, self.year, month).unwrap())
            .collect()
    }

    fn first_month_number(&self) -> i32 {
        if self.year == 1 { self.era.start_ymd().1 } else { 1 }
    }

    fn last_month_number(&self) -> i32 {
        if self.year == self.era.max_year() { self.era.end_ymd().map_or(12, |(_, month, _)| month) } else { 12 }
    }

    pub fn day_count(&self) -> i32 {
        self.months().iter().map(JapaneseMonth::day_count).sum()
    }

    pub fn first_day(&self) -> Japanese {
        self.first_month().first_day()
    }

    pub fn last_day(&self) -> Japanese {
        self.last_month().last_day()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn next(&self, years: i32) -> Result<Self, LunarError> {
        Self::from_solar(Solar::from_ymd(self.solar_year() + years, 1, 1)?)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_japanese(&self, japanese: Japanese) -> bool {
        japanese.japanese_year() == *self
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

impl CalendarSpan for JapaneseYear {
    fn first_solar_day(&self) -> Solar {
        JapaneseYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        JapaneseYear::last_solar_day(self)
    }
}

impl fmt::Display for JapaneseYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}年", self.era, self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JapaneseMonth {
    era: JapaneseEra,
    year: i32,
    month: i32,
}

impl JapaneseMonth {
    pub fn from_eym(era: JapaneseEra, year: i32, month: i32) -> Result<Self, LunarError> {
        let year_obj = JapaneseYear::from_era_year(era, year)?;
        if month < year_obj.first_month_number() || month > year_obj.last_month_number() {
            return Err(LunarError::Parse(format!("illegal japanese month: {}{}-{}", era.name(), year, month)));
        }
        Ok(Self { era, year, month })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let date = Japanese::from_solar(solar)?;
        Self::from_eym(date.era(), date.year(), date.month())
    }

    pub const fn era(&self) -> JapaneseEra {
        self.era
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn solar_year(&self) -> i32 {
        self.era.start_ymd().0 + self.year - 1
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn japanese_year(&self) -> JapaneseYear {
        JapaneseYear::from_era_year(self.era, self.year).unwrap()
    }

    fn first_day_number(&self) -> i32 {
        if self.year == 1 && self.month == self.era.start_ymd().1 { self.era.start_ymd().2 } else { 1 }
    }

    fn last_day_number(&self) -> i32 {
        if self.year == self.era.max_year()
            && let Some((_, month, day)) = self.era.end_ymd()
            && self.month == month
        {
            day
        } else {
            crate::solar_util::days_of_month(self.solar_year(), self.month)
        }
    }

    pub fn day_count(&self) -> i32 {
        self.last_day_number() - self.first_day_number() + 1
    }

    pub fn first_day(&self) -> Japanese {
        Japanese::from_eymd(self.era, self.year, self.month, self.first_day_number()).unwrap()
    }

    pub fn last_day(&self) -> Japanese {
        Japanese::from_eymd(self.era, self.year, self.month, self.last_day_number()).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<Japanese> {
        (self.first_day_number()..=self.last_day_number())
            .map(|day| Japanese::from_eymd(self.era, self.year, self.month, day).unwrap())
            .collect()
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        let (year, month) = next_ym(self.solar_year(), self.month, months);
        Self::from_solar(Solar::from_ymd(year, month, 1)?)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_japanese(&self, japanese: Japanese) -> bool {
        japanese.era() == self.era && japanese.year() == self.year && japanese.month() == self.month
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

impl CalendarSpan for JapaneseMonth {
    fn first_solar_day(&self) -> Solar {
        JapaneseMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        JapaneseMonth::last_solar_day(self)
    }
}

impl fmt::Display for JapaneseMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}月", self.japanese_year(), self.month)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Japanese {
    era: JapaneseEra,
    year: i32,
    month: i32,
    day: i32,
}

impl Japanese {
    pub fn from_eymd(era: JapaneseEra, year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        let month_obj = JapaneseMonth::from_eym(era, year, month)?;
        if day < month_obj.first_day_number() || day > month_obj.last_day_number() {
            return Err(LunarError::Parse(format!("illegal japanese day: {}{}-{}-{}", era.name(), year, month, day)));
        }
        Ok(Self { era, year, month, day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let era = JapaneseEra::from_solar(solar)?;
        Ok(Self { era, year: solar.year() - era.start_ymd().0 + 1, month: solar.month(), day: solar.day() })
    }

    pub const fn era(&self) -> JapaneseEra {
        self.era
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub fn solar_year(&self) -> i32 {
        self.era.start_ymd().0 + self.year - 1
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub fn japanese_year(&self) -> JapaneseYear {
        JapaneseYear::from_era_year(self.era, self.year).unwrap()
    }

    pub fn japanese_month(&self) -> JapaneseMonth {
        JapaneseMonth::from_eym(self.era, self.year, self.month).unwrap()
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
        crate::i18n::locale(language).render_japanese_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!("{}，公历{}，星期{}", self.to_string_cn(), self.solar().to_ymd(), self.solar().week_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_japanese_full(self)
    }
}

impl CalendarDay for Japanese {
    fn solar(&self) -> Solar {
        Japanese::solar(self)
    }
}

impl fmt::Display for Japanese {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}年{}月{}日", self.era, self.year, self.month, self.day)
    }
}
