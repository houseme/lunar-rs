//! 道历。对应 lunar-go `calendar/Tao.go`。

use std::fmt;
use std::marker::PhantomData;

use crate::event::{CalendarKind, Event, EventKind, EventQuery, EventSource, TaoFestivalEvent, filter_events};
use crate::lunar::Lunar;
use crate::lunar_month::LunarMonth;
use crate::lunar_util;
use crate::lunar_year::LunarYear;
use crate::multi_calendar::{CalendarSpan, span_all_events, span_contains_solar, span_events, span_find_events};
use crate::solar::Solar;
use crate::tao_util;

/// 道历元年（公元前 2697 年，老子诞辰）。
pub const BIRTH_YEAR: i32 = -2697;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaoYear {
    year: i32,
}

impl TaoYear {
    pub const fn from_year(year: i32) -> Self {
        Self { year }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn get_year(&self) -> i32 {
        self.year()
    }

    pub const fn lunar_year(&self) -> i32 {
        self.year + BIRTH_YEAR
    }

    pub const fn get_lunar_year(&self) -> i32 {
        self.lunar_year()
    }

    pub fn first_month(&self) -> TaoMonth {
        let month = LunarYear::from_year(self.lunar_year()).months_in_year().next().unwrap();
        TaoMonth::from_lunar_month(month)
    }

    pub fn get_first_month(&self) -> TaoMonth {
        self.first_month()
    }

    pub fn last_month(&self) -> TaoMonth {
        let month = LunarYear::from_year(self.lunar_year()).months_in_year().last().unwrap();
        TaoMonth::from_lunar_month(month)
    }

    pub fn get_last_month(&self) -> TaoMonth {
        self.last_month()
    }

    pub fn months(&self) -> Vec<TaoMonth> {
        LunarYear::from_year(self.lunar_year()).months_in_year().map(TaoMonth::from_lunar_month).collect()
    }

    pub fn get_months(&self) -> Vec<TaoMonth> {
        self.months()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_month().first_solar_day()
    }

    pub fn get_first_solar_day(&self) -> Solar {
        self.first_solar_day()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_month().last_solar_day()
    }

    pub fn get_last_solar_day(&self) -> Solar {
        self.last_solar_day()
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
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
}

impl CalendarSpan for TaoYear {
    fn first_solar_day(&self) -> Solar {
        TaoYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        TaoYear::last_solar_day(self)
    }
}

impl fmt::Display for TaoYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaoMonth {
    lunar_year: i32,
    month: i32,
    day_count: i32,
    first_julian_day_bits: u64,
    index: i32,
}

impl TaoMonth {
    pub const fn from_lunar_month(lunar_month: LunarMonth) -> Self {
        Self {
            lunar_year: lunar_month.year(),
            month: lunar_month.month(),
            day_count: lunar_month.day_count(),
            first_julian_day_bits: lunar_month.first_julian_day().to_bits(),
            index: lunar_month.index(),
        }
    }

    pub const fn year(&self) -> i32 {
        self.lunar_year - BIRTH_YEAR
    }

    pub const fn get_year(&self) -> i32 {
        self.year()
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn get_month(&self) -> i32 {
        self.month()
    }

    pub const fn is_leap(&self) -> bool {
        self.month < 0
    }

    pub const fn day_count(&self) -> i32 {
        self.day_count
    }

    pub const fn get_day_count(&self) -> i32 {
        self.day_count()
    }

    pub const fn index(&self) -> i32 {
        self.index
    }

    pub const fn get_index(&self) -> i32 {
        self.index()
    }

    pub fn name(&self) -> String {
        if self.month() < 0 {
            format!("闰{}", lunar_util::tables::MONTH[self.month().unsigned_abs() as usize])
        } else {
            lunar_util::tables::MONTH[self.month() as usize].to_string()
        }
    }

    pub fn get_name(&self) -> String {
        self.name()
    }

    pub fn first_solar_day(&self) -> Solar {
        Solar::from_julian_day(f64::from_bits(self.first_julian_day_bits))
    }

    pub fn get_first_solar_day(&self) -> Solar {
        self.first_solar_day()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.first_solar_day().next_day(self.day_count() - 1)
    }

    pub fn get_last_solar_day(&self) -> Solar {
        self.last_solar_day()
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn next(&self, months: i32) -> Option<Self> {
        LunarMonth::from_ym(self.lunar_year, self.month)
            .and_then(|lunar_month| lunar_month.next(months))
            .map(Self::from_lunar_month)
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
}

impl CalendarSpan for TaoMonth {
    fn first_solar_day(&self) -> Solar {
        TaoMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        TaoMonth::last_solar_day(self)
    }
}

impl fmt::Display for TaoMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年{}月", self.year(), self.name())
    }
}

/// 道历节日记录。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TaoFestival {
    name: &'static str,
    remark: &'static str,
}

impl TaoFestival {
    pub(crate) fn new(name: &'static str, remark: &'static str) -> Self {
        Self { name, remark }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn remark(&self) -> &str {
        self.remark
    }

    pub fn to_event(&self, solar: crate::Solar) -> Event {
        if self.remark().is_empty() {
            Event::with_meta(
                EventKind::TaoFestival,
                CalendarKind::Tao,
                EventSource::BuiltInFestival,
                self.name,
                solar,
                None::<crate::event::EventDetail>,
                90,
                Some(crate::event::EventSourceId::NamedSolar { prefix: "tao", solar }),
                true,
                true,
                ["tao", "festival", "single_day"],
            )
        } else {
            Event::with_meta(
                EventKind::TaoFestival,
                CalendarKind::Tao,
                EventSource::BuiltInFestival,
                self.name,
                solar,
                Some(crate::event::EventDetail::Remark { remark: self.remark.into() }),
                90,
                Some(crate::event::EventSourceId::NamedSolar { prefix: "tao", solar }),
                true,
                true,
                ["tao", "festival", "remarked"],
            )
        }
    }
}

impl fmt::Display for TaoFestival {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.remark.is_empty() { write!(f, "{}", self.name) } else { write!(f, "{} {}", self.name, self.remark) }
    }
}

/// 道历。内部持有一个 [`Lunar`] 快照，兼容对外的 lifetime 形状。
#[derive(Clone)]
pub struct Tao<'a> {
    lunar: Lunar,
    marker: PhantomData<&'a Lunar>,
}

impl<'a> Tao<'a> {
    pub(crate) fn from_lunar(lunar: &'a Lunar) -> Tao<'static> {
        Tao { lunar: lunar.clone(), marker: PhantomData }
    }

    pub const fn lunar(&self) -> &Lunar {
        &self.lunar
    }
    pub fn get_lunar(&self) -> &Lunar {
        self.lunar()
    }
    pub const fn year(&self) -> i32 {
        self.lunar.year() - BIRTH_YEAR
    }
    pub const fn get_year(&self) -> i32 {
        self.year()
    }
    pub const fn month(&self) -> i32 {
        self.lunar.month()
    }
    pub const fn get_month(&self) -> i32 {
        self.month()
    }
    pub const fn day(&self) -> i32 {
        self.lunar.day()
    }
    pub const fn get_day(&self) -> i32 {
        self.day()
    }

    pub fn tao_year(&self) -> TaoYear {
        TaoYear::from_year(self.year())
    }
    pub fn get_tao_year(&self) -> TaoYear {
        self.tao_year()
    }

    pub fn tao_month(&self) -> TaoMonth {
        let month = LunarMonth::from_ym(self.lunar.year(), self.lunar.month()).unwrap();
        TaoMonth::from_lunar_month(month)
    }
    pub fn get_tao_month(&self) -> TaoMonth {
        self.tao_month()
    }

    pub fn year_in_chinese(&self) -> String {
        self.year()
            .to_string()
            .chars()
            .map(|c| lunar_util::tables::NUMBER[c.to_digit(10).unwrap_or(0) as usize])
            .collect()
    }
    pub fn get_year_in_chinese(&self) -> String {
        self.year_in_chinese()
    }
    pub fn month_in_chinese(&self) -> String {
        self.lunar.month_in_chinese()
    }
    pub fn get_month_in_chinese(&self) -> String {
        self.month_in_chinese()
    }
    pub fn day_in_chinese(&self) -> &'static str {
        self.lunar.day_in_chinese()
    }
    pub fn get_day_in_chinese(&self) -> &'static str {
        self.day_in_chinese()
    }

    pub fn festivals(&self) -> Vec<TaoFestival> {
        let mut out = Vec::new();
        for festival in tao_util::festivals(self.month(), self.day()) {
            let remark = festival.get(1).copied().unwrap_or("");
            out.push(TaoFestival::new(festival.first().copied().unwrap_or(""), remark));
        }
        let jq = self.lunar.jie_qi();
        if jq == "冬至" {
            out.push(TaoFestival::new("元始天尊圣诞", ""));
        } else if jq == "夏至" {
            out.push(TaoFestival::new("灵宝天尊圣诞", ""));
        }
        if let Some(f) = tao_util::BA_JIE.get(jq) {
            out.push(TaoFestival::new(f, ""));
        }
        if let Some(f) = tao_util::BA_HUI.get(self.lunar.day_in_gan_zhi().as_str()) {
            out.push(TaoFestival::new(f, ""));
        }
        out
    }
    pub fn get_festivals(&self) -> Vec<TaoFestival> {
        self.festivals()
    }

    /// Unified events for the current Taoist calendar date.
    pub fn events(&self) -> Vec<Event> {
        let solar = self.lunar.solar();
        self.festivals().into_iter().map(|festival| TaoFestivalEvent::new(festival, solar).to_event()).collect()
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        filter_events(&self.events(), query)
    }

    pub fn is_day_san_hui(&self) -> bool {
        tao_util::is_day_san_hui(self.month(), self.day())
    }
    pub fn get_is_day_san_hui(&self) -> bool {
        self.is_day_san_hui()
    }
    pub fn is_day_san_yuan(&self) -> bool {
        tao_util::is_day_san_yuan(self.month(), self.day())
    }
    pub fn get_is_day_san_yuan(&self) -> bool {
        self.is_day_san_yuan()
    }
    pub fn is_day_wu_la(&self) -> bool {
        tao_util::is_day_wu_la(self.month(), self.day())
    }
    pub fn get_is_day_wu_la(&self) -> bool {
        self.is_day_wu_la()
    }
    pub fn is_day_ba_jie(&self) -> bool {
        tao_util::BA_JIE.contains_key(self.lunar.jie_qi())
    }
    pub fn get_is_day_ba_jie(&self) -> bool {
        self.is_day_ba_jie()
    }
    pub fn is_day_ba_hui(&self) -> bool {
        tao_util::BA_HUI.contains_key(self.lunar.day_in_gan_zhi().as_str())
    }
    pub fn get_is_day_ba_hui(&self) -> bool {
        self.is_day_ba_hui()
    }
    pub fn is_day_ming_wu(&self) -> bool {
        self.lunar.day_gan() == "戊"
    }
    pub fn get_is_day_ming_wu(&self) -> bool {
        self.is_day_ming_wu()
    }
    pub fn is_day_an_wu(&self) -> bool {
        let m = self.month().unsigned_abs() as usize;
        tao_util::AN_WU.get(m - 1).copied().unwrap_or("") == self.lunar.day_zhi()
    }
    pub fn get_is_day_an_wu(&self) -> bool {
        self.is_day_an_wu()
    }
    pub fn is_day_wu(&self) -> bool {
        self.is_day_ming_wu() || self.is_day_an_wu()
    }
    pub fn get_is_day_wu(&self) -> bool {
        self.is_day_wu()
    }

    pub fn to_string_cn(&self) -> String {
        format!("{}年{}月{}", self.year_in_chinese(), self.month_in_chinese(), self.day_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_tao_string(self)
    }

    pub fn to_full_string(&self) -> String {
        format!(
            "道歷{}年，天運{}年，{}月，{}日。{}月{}日，{}时。",
            self.year_in_chinese(),
            self.lunar.year_in_gan_zhi(),
            self.lunar.month_in_gan_zhi(),
            self.lunar.day_in_gan_zhi(),
            self.month_in_chinese(),
            self.day_in_chinese(),
            self.lunar.time_zhi()
        )
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_tao_full(self)
    }
}
