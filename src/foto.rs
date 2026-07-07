//! 佛历。对应 lunar-go `calendar/Foto.go`。

use std::fmt;
use std::fmt::Write as _;
use std::marker::PhantomData;

use crate::event::{CalendarKind, Event, EventKind, EventQuery, EventSource, FotoFestivalEvent, filter_events};
use crate::foto_util;
use crate::lunar::Lunar;
use crate::lunar_month::LunarMonth;
use crate::lunar_util;
use crate::lunar_year::LunarYear;
use crate::multi_calendar::{CalendarSpan, span_all_events, span_contains_solar, span_events, span_find_events};
use crate::solar::Solar;

/// 佛历元年（公元前 543 年）。
pub const DEAD_YEAR: i32 = -543;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FotoYear {
    year: i32,
}

impl FotoYear {
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
        self.year + DEAD_YEAR - 1
    }

    pub const fn get_lunar_year(&self) -> i32 {
        self.lunar_year()
    }

    pub fn first_month(&self) -> FotoMonth {
        let month = LunarYear::from_year(self.lunar_year()).months_in_year().next().unwrap();
        FotoMonth::from_lunar_month(month)
    }

    pub fn get_first_month(&self) -> FotoMonth {
        self.first_month()
    }

    pub fn last_month(&self) -> FotoMonth {
        let month = LunarYear::from_year(self.lunar_year()).months_in_year().last().unwrap();
        FotoMonth::from_lunar_month(month)
    }

    pub fn get_last_month(&self) -> FotoMonth {
        self.last_month()
    }

    pub fn months(&self) -> Vec<FotoMonth> {
        LunarYear::from_year(self.lunar_year()).months_in_year().map(FotoMonth::from_lunar_month).collect()
    }

    pub fn get_months(&self) -> Vec<FotoMonth> {
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

impl CalendarSpan for FotoYear {
    fn first_solar_day(&self) -> Solar {
        FotoYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        FotoYear::last_solar_day(self)
    }
}

impl fmt::Display for FotoYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年", self.year)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FotoMonth {
    lunar_year: i32,
    month: i32,
    day_count: i32,
    first_julian_day_bits: u64,
    index: i32,
}

impl FotoMonth {
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
        self.lunar_year - DEAD_YEAR + 1
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

impl CalendarSpan for FotoMonth {
    fn first_solar_day(&self) -> Solar {
        FotoMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        FotoMonth::last_solar_day(self)
    }
}

impl fmt::Display for FotoMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年{}月", self.year(), self.name())
    }
}

/// 佛历节日记录。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FotoFestival {
    name: &'static str,
    result: &'static str,
    every_month: bool,
    remark: &'static str,
}

impl FotoFestival {
    fn from_record(o: &[&'static str]) -> Self {
        let name = o.first().copied().unwrap_or("");
        let result = o.get(1).copied().unwrap_or("");
        let every_month = o.get(2).is_some_and(|x| *x == "true");
        let remark = o.get(3).copied().unwrap_or("");
        Self { name, result, every_month, remark }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn result(&self) -> &str {
        self.result
    }
    pub const fn every_month(&self) -> bool {
        self.every_month
    }
    pub fn remark(&self) -> &str {
        self.remark
    }

    pub fn to_event(&self, solar: crate::Solar) -> Event {
        Event::with_meta(
            EventKind::FotoFestival,
            CalendarKind::Foto,
            EventSource::BuiltInFestival,
            self.name,
            solar,
            Some(crate::event::EventDetail::FotoFestival {
                result: self.result.into(),
                remark: (!self.remark.is_empty()).then(|| self.remark.into()),
                every_month: self.every_month(),
            }),
            70,
            Some(crate::event::EventSourceId::NamedSolar { prefix: "foto", solar }),
            true,
            true,
            ["foto", "festival", if self.every_month() { "recurring" } else { "single_day" }],
        )
    }
}

impl fmt::Display for FotoFestival {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.remark.is_empty() {
            write!(f, "{} {}", self.name, self.result)
        } else {
            write!(f, "{} {} {}", self.name, self.result, self.remark)
        }
    }
}

/// 佛历。内部持有一个 [`Lunar`] 快照，兼容对外的 lifetime 形状。
#[derive(Clone)]
pub struct Foto<'a> {
    lunar: Lunar,
    marker: PhantomData<&'a Lunar>,
}

impl<'a> Foto<'a> {
    pub(crate) fn from_lunar(lunar: &'a Lunar) -> Foto<'static> {
        Foto { lunar: lunar.clone(), marker: PhantomData }
    }

    pub const fn lunar(&self) -> &Lunar {
        &self.lunar
    }
    pub fn get_lunar(&self) -> &Lunar {
        self.lunar()
    }
    pub const fn year(&self) -> i32 {
        let sy = self.lunar.solar().year();
        let mut y = sy - DEAD_YEAR;
        if sy == self.lunar.year() {
            y += 1;
        }
        y
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

    pub fn foto_year(&self) -> FotoYear {
        FotoYear::from_year(self.year())
    }
    pub fn get_foto_year(&self) -> FotoYear {
        self.foto_year()
    }

    pub fn foto_month(&self) -> FotoMonth {
        let month = LunarMonth::from_ym(self.lunar.year(), self.lunar.month()).unwrap();
        FotoMonth::from_lunar_month(month)
    }
    pub fn get_foto_month(&self) -> FotoMonth {
        self.foto_month()
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

    pub fn festivals(&self) -> Vec<FotoFestival> {
        let m = self.month().abs();
        let mut out = Vec::new();
        for festival in foto_util::festivals(m, self.day()) {
            out.push(FotoFestival::from_record(festival));
        }
        out
    }
    pub fn get_festivals(&self) -> Vec<FotoFestival> {
        self.festivals()
    }
    pub fn other_festivals(&self) -> Vec<&'static str> {
        foto_util::other_festivals(self.month(), self.day()).to_vec()
    }
    pub fn get_other_festivals(&self) -> Vec<&'static str> {
        self.other_festivals()
    }

    /// Unified events for the current Buddhist calendar date.
    pub fn events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        let solar = self.lunar.solar();

        for festival in self.festivals() {
            events.push(FotoFestivalEvent::new(festival, solar).to_event());
        }

        for name in self.other_festivals() {
            events.push(Event::new(
                EventKind::FotoOtherFestival,
                CalendarKind::Foto,
                EventSource::BuiltInOtherFestival,
                name,
                solar,
            ));
        }

        events
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        filter_events(&self.events(), query)
    }

    pub const fn is_month_zhai(&self) -> bool {
        let m = self.month();
        m == 1 || m == 5 || m == 9
    }
    pub const fn get_is_month_zhai(&self) -> bool {
        self.is_month_zhai()
    }
    pub fn is_day_yang_gong(&self) -> bool {
        self.festivals().iter().any(|f| f.name() == "杨公忌")
    }
    pub fn get_is_day_yang_gong(&self) -> bool {
        self.is_day_yang_gong()
    }
    pub const fn is_day_zhai_shuo_wang(&self) -> bool {
        let d = self.day();
        d == 1 || d == 15
    }
    pub const fn get_is_day_zhai_shuo_wang(&self) -> bool {
        self.is_day_zhai_shuo_wang()
    }
    pub fn is_day_zhai_six(&self) -> bool {
        let d = self.day();
        if d == 8 || d == 14 || d == 15 || d == 23 || d == 29 || d == 30 {
            return true;
        }
        if d == 28
            && let Some(m) = LunarMonth::from_ym(self.lunar.year(), self.month())
        {
            return m.day_count() != 30;
        }
        false
    }
    pub fn get_is_day_zhai_six(&self) -> bool {
        self.is_day_zhai_six()
    }
    pub const fn is_day_zhai_ten(&self) -> bool {
        let d = self.day();
        d == 1 || d == 8 || d == 14 || d == 15 || d == 18 || d == 23 || d == 24 || d == 28 || d == 29 || d == 30
    }
    pub const fn get_is_day_zhai_ten(&self) -> bool {
        self.is_day_zhai_ten()
    }
    pub fn is_day_zhai_guan_yin(&self) -> bool {
        foto_util::is_day_zhai_guan_yin(self.month(), self.day())
    }
    pub fn get_is_day_zhai_guan_yin(&self) -> bool {
        self.is_day_zhai_guan_yin()
    }

    pub fn xiu(&self) -> &'static str {
        foto_util::get_xiu(self.month(), self.day())
    }
    pub fn get_xiu(&self) -> &'static str {
        self.xiu()
    }
    pub fn xiu_luck(&self) -> &'static str {
        lunar_util::xiu_luck(self.xiu())
    }
    pub fn get_xiu_luck(&self) -> &'static str {
        self.xiu_luck()
    }
    pub fn xiu_song(&self) -> &'static str {
        lunar_util::xiu_song(self.xiu())
    }
    pub fn get_xiu_song(&self) -> &'static str {
        self.xiu_song()
    }
    pub fn zheng(&self) -> &'static str {
        lunar_util::zheng(self.xiu())
    }
    pub fn get_zheng(&self) -> &'static str {
        self.zheng()
    }
    pub fn animal(&self) -> &'static str {
        lunar_util::animal(self.xiu())
    }
    pub fn get_animal(&self) -> &'static str {
        self.animal()
    }
    pub fn gong(&self) -> &'static str {
        lunar_util::gong(self.xiu())
    }
    pub fn get_gong(&self) -> &'static str {
        self.gong()
    }
    pub fn shou(&self) -> &'static str {
        lunar_util::shou(self.gong())
    }
    pub fn get_shou(&self) -> &'static str {
        self.shou()
    }

    pub fn to_string_cn(&self) -> String {
        format!("{}年{}月{}", self.year_in_chinese(), self.month_in_chinese(), self.day_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_foto_string(self)
    }

    pub fn to_full_string(&self) -> String {
        let mut s = self.to_string_cn();
        for f in self.festivals() {
            let _ = write!(s, " ({f})");
        }
        s
    }

    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_foto_full(self)
    }
}
