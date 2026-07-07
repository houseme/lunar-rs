//! 节气对象（名称 + 对应阳历时刻）。

use std::fmt;

use crate::JulianDay;
use crate::event::{EventDetail, EventSourceId};
use crate::lunar_year::JIE_QI;
use crate::shou_xing;
use crate::solar::Solar;
use crate::{CalendarKind, Event, EventKind, EventSource};

/// 一个节气 / 节令 / 气令及其阳历时刻。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct JieQi {
    name: String,
    solar: Solar,
    year: i32,
    index: usize,
    cursory_julian_day: f64,
}

impl JieQi {
    pub(crate) fn from_solar(name: impl Into<String>, solar: Solar) -> Self {
        let name = name.into();
        let index = term_index(&name);
        let year = term_year_from_solar(solar, index);
        Self { name, solar, year, index, cursory_julian_day: cursory_julian_day(year, index) }
    }

    pub fn new(year: i32, name: &str) -> Option<Self> {
        Self::from_name(year, name)
    }

    pub fn from_name(year: i32, name: &str) -> Option<Self> {
        JIE_QI.iter().position(|value| *value == name).map(|index| Self::from_index(year, index as isize))
    }

    pub fn from_index(year: i32, index: isize) -> Self {
        let size = JIE_QI.len() as isize;
        let year = year + index.div_euclid(size) as i32;
        let index = index.rem_euclid(size) as usize;
        let cursory_julian_day = cursory_julian_day(year, index);
        let solar = Solar::from_julian_day(accurate_julian_day_from_cursory(cursory_julian_day));
        Self { name: JIE_QI[index].to_string(), solar, year, index, cursory_julian_day }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    pub const fn solar(&self) -> Solar {
        self.solar
    }

    pub const fn solar_time(&self) -> Solar {
        self.solar
    }

    pub fn solar_day(&self) -> Solar {
        Solar::from_ymd(self.solar.year(), self.solar.month(), self.solar.day()).unwrap_or(self.solar)
    }

    pub fn get_solar_day(&self) -> Solar {
        self.solar_day()
    }

    pub fn julian_day(&self) -> JulianDay {
        JulianDay::from_julian_day(self.solar.julian_day())
    }

    pub fn get_julian_day(&self) -> JulianDay {
        self.julian_day()
    }

    pub fn cursory_julian_day(&self) -> f64 {
        self.cursory_julian_day
    }

    pub fn get_cursory_julian_day(&self) -> f64 {
        self.cursory_julian_day()
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn get_year(&self) -> i32 {
        self.year()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn get_index(&self) -> usize {
        self.index()
    }

    pub fn is_jie(&self) -> bool {
        self.index() % 2 == 1
    }

    pub fn is_qi(&self) -> bool {
        self.index() % 2 == 0
    }

    pub fn next(&self, offset: isize) -> Self {
        Self::from_index(self.year(), self.index() as isize + offset)
    }

    /// 节气名称（显式语言版本，需启用 `i18n` feature）。
    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::jieqi(self.name(), language)
    }

    pub fn to_event(&self, calendar_kind: CalendarKind) -> Event {
        Event::with_meta(
            EventKind::JieQi,
            calendar_kind,
            EventSource::JieQi,
            self.name().to_string(),
            self.solar,
            Some(EventDetail::JieQi { at: self.solar }),
            10,
            Some(EventSourceId::JieQi { at: self.solar }),
            true,
            true,
            [
                "jieqi",
                "seasonal",
                match calendar_kind {
                    CalendarKind::Solar => "solar",
                    CalendarKind::Lunar => "lunar",
                    CalendarKind::Foto => "foto",
                    CalendarKind::Tao => "tao",
                },
            ],
        )
    }
}

impl fmt::Display for JieQi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl PartialEq for JieQi {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for JieQi {}

fn cursory_julian_day(year: i32, index: usize) -> f64 {
    let jd = (f64::from(year - 2000) * 365.2422 + 180.0).floor();
    let mut w = ((jd - 355.0 + 183.0) / 365.2422).floor().mul_add(365.2422, 355.0);
    if shou_xing::calc_qi(w) > jd {
        w -= 365.2422;
    }
    shou_xing::calc_qi(15.2184f64.mul_add(index as f64, w))
}

fn accurate_julian_day_from_cursory(cursory_julian_day: f64) -> f64 {
    shou_xing::qi_accurate_2(cursory_julian_day) + shou_xing::J2000
}

fn term_index(name: &str) -> usize {
    JIE_QI.iter().position(|value| *value == name).unwrap_or(0)
}

fn term_year_from_solar(solar: Solar, index: usize) -> i32 {
    if index == 0 && solar.month() == 12 { solar.year() + 1 } else { solar.year() }
}
