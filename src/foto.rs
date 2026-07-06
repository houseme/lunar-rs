//! 佛历。对应 lunar-go `calendar/Foto.go`。

use std::fmt;
use std::fmt::Write as _;

use crate::event::{Event, EventKind};
use crate::foto_util;
use crate::lunar::Lunar;
use crate::lunar_month::LunarMonth;
use crate::lunar_util;

/// 佛历元年（公元前 543 年）。
pub const DEAD_YEAR: i32 = -543;

/// 佛历节日记录。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FotoFestival {
    name: String,
    result: String,
    every_month: bool,
    remark: String,
}

impl FotoFestival {
    fn from_record(o: &[&str]) -> Self {
        let name = o.first().unwrap_or(&"").to_string();
        let result = o.get(1).unwrap_or(&"").to_string();
        let every_month = o.get(2).is_some_and(|x| *x == "true");
        let remark = o.get(3).unwrap_or(&"").to_string();
        Self { name, result, every_month, remark }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn result(&self) -> &str {
        &self.result
    }
    pub const fn every_month(&self) -> bool {
        self.every_month
    }
    pub fn remark(&self) -> &str {
        &self.remark
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

/// 佛历。借用底层 [`Lunar`]。
pub struct Foto<'a> {
    lunar: &'a Lunar,
}

impl<'a> Foto<'a> {
    pub(crate) const fn from_lunar(lunar: &'a Lunar) -> Self {
        Self { lunar }
    }

    pub const fn lunar(&self) -> &Lunar {
        self.lunar
    }
    pub const fn year(&self) -> i32 {
        let sy = self.lunar.solar().year();
        let mut y = sy - DEAD_YEAR;
        if sy == self.lunar.year() {
            y += 1;
        }
        y
    }
    pub const fn month(&self) -> i32 {
        self.lunar.month()
    }
    pub const fn day(&self) -> i32 {
        self.lunar.day()
    }

    pub fn year_in_chinese(&self) -> String {
        self.year()
            .to_string()
            .chars()
            .map(|c| lunar_util::tables::NUMBER[c.to_digit(10).unwrap_or(0) as usize])
            .collect()
    }
    pub fn month_in_chinese(&self) -> String {
        self.lunar.month_in_chinese()
    }
    pub fn day_in_chinese(&self) -> &'static str {
        self.lunar.day_in_chinese()
    }

    pub fn festivals(&self) -> Vec<FotoFestival> {
        let m = self.month().abs();
        let key = format!("{m}-{}", self.day());
        let mut out = Vec::new();
        if let Some(fs) = foto_util::FESTIVAL.get(key.as_str()) {
            for o in fs {
                out.push(FotoFestival::from_record(o));
            }
        }
        out
    }
    pub fn other_festivals(&self) -> Vec<&'static str> {
        let key = format!("{}-{}", self.month(), self.day());
        foto_util::OTHER_FESTIVAL.get(key.as_str()).cloned().unwrap_or_default()
    }

    /// Unified events for the current Buddhist calendar date.
    pub fn events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        let solar = self.lunar.solar();

        for festival in self.festivals() {
            let detail = if festival.remark().is_empty() {
                festival.result().to_string()
            } else {
                format!("{} {}", festival.result(), festival.remark())
            };
            events.push(Event::with_detail(EventKind::FotoFestival, festival.name(), solar, detail));
        }

        for name in self.other_festivals() {
            events.push(Event::new(EventKind::FotoOtherFestival, name, solar));
        }

        events
    }

    pub const fn is_month_zhai(&self) -> bool {
        let m = self.month();
        m == 1 || m == 5 || m == 9
    }
    pub fn is_day_yang_gong(&self) -> bool {
        self.festivals().iter().any(|f| f.name() == "杨公忌")
    }
    pub const fn is_day_zhai_shuo_wang(&self) -> bool {
        let d = self.day();
        d == 1 || d == 15
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
    pub const fn is_day_zhai_ten(&self) -> bool {
        let d = self.day();
        d == 1 || d == 8 || d == 14 || d == 15 || d == 18 || d == 23 || d == 24 || d == 28 || d == 29 || d == 30
    }
    pub fn is_day_zhai_guan_yin(&self) -> bool {
        foto_util::is_day_zhai_guan_yin(&format!("{}-{}", self.month(), self.day()))
    }

    pub fn xiu(&self) -> &'static str {
        foto_util::get_xiu(self.month(), self.day())
    }
    pub fn xiu_luck(&self) -> &'static str {
        lunar_util::xiu_luck(self.xiu())
    }
    pub fn xiu_song(&self) -> &'static str {
        lunar_util::xiu_song(self.xiu())
    }
    pub fn zheng(&self) -> &'static str {
        lunar_util::zheng(self.xiu())
    }
    pub fn animal(&self) -> &'static str {
        lunar_util::animal(self.xiu())
    }
    pub fn gong(&self) -> &'static str {
        lunar_util::gong(self.xiu())
    }
    pub fn shou(&self) -> &'static str {
        lunar_util::shou(self.gong())
    }

    pub fn to_string_cn(&self) -> String {
        format!("{}年{}月{}", self.year_in_chinese(), self.month_in_chinese(), self.day_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_string_in_lang(&self, language: crate::i18n::Language) -> String {
        match language {
            crate::i18n::Language::ZhCn => self.to_string_cn(),
            crate::i18n::Language::En => {
                format!("Buddhist {}-{:02}-{:02}", self.year(), self.month().abs(), self.day())
            }
        }
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
        if matches!(language, crate::i18n::Language::ZhCn) {
            return self.to_full_string();
        }
        let mut s = self.to_string_in_lang(language);
        for f in self.festivals() {
            let _ = write!(s, " ({f})");
        }
        s
    }
}
