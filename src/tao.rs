//! 道历。对应 lunar-go `calendar/Tao.go`。

use std::fmt;

use crate::lunar::Lunar;
use crate::lunar_util;
use crate::tao_util;

/// 道历元年（公元前 2697 年，老子诞辰）。
pub const BIRTH_YEAR: i32 = -2697;

/// 道历节日记录。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct TaoFestival {
    name: String,
    remark: String,
}

impl TaoFestival {
    pub(crate) fn new(name: &str, remark: &str) -> Self {
        Self { name: name.to_string(), remark: remark.to_string() }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn remark(&self) -> &str {
        &self.remark
    }
}

impl fmt::Display for TaoFestival {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.remark.is_empty() { write!(f, "{}", self.name) } else { write!(f, "{} {}", self.name, self.remark) }
    }
}

/// 道历。借用底层 [`Lunar`]。
pub struct Tao<'a> {
    lunar: &'a Lunar,
}

impl<'a> Tao<'a> {
    pub(crate) const fn from_lunar(lunar: &'a Lunar) -> Self {
        Self { lunar }
    }

    pub const fn lunar(&self) -> &Lunar {
        self.lunar
    }
    pub const fn year(&self) -> i32 {
        self.lunar.year() - BIRTH_YEAR
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

    pub fn festivals(&self) -> Vec<TaoFestival> {
        let key = format!("{}-{}", self.month(), self.day());
        let mut out = Vec::new();
        if let Some(fs) = tao_util::FESTIVAL.get(key.as_str()) {
            for o in fs {
                let remark = o.get(1).copied().unwrap_or("");
                out.push(TaoFestival::new(o.first().copied().unwrap_or(""), remark));
            }
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

    fn is_day_in(&self, days: &[&str]) -> bool {
        let k = format!("{}-{}", self.month(), self.day());
        days.iter().any(|x| *x == k)
    }
    pub fn is_day_san_hui(&self) -> bool {
        self.is_day_in(tao_util::SAN_HUI)
    }
    pub fn is_day_san_yuan(&self) -> bool {
        self.is_day_in(tao_util::SAN_YUAN)
    }
    pub fn is_day_wu_la(&self) -> bool {
        self.is_day_in(tao_util::WU_LA)
    }
    pub fn is_day_ba_jie(&self) -> bool {
        tao_util::BA_JIE.contains_key(self.lunar.jie_qi())
    }
    pub fn is_day_ba_hui(&self) -> bool {
        tao_util::BA_HUI.contains_key(self.lunar.day_in_gan_zhi().as_str())
    }
    pub fn is_day_ming_wu(&self) -> bool {
        self.lunar.day_gan() == "戊"
    }
    pub fn is_day_an_wu(&self) -> bool {
        let m = self.month().unsigned_abs() as usize;
        tao_util::AN_WU.get(m - 1).copied().unwrap_or("") == self.lunar.day_zhi()
    }
    pub fn is_day_wu(&self) -> bool {
        self.is_day_ming_wu() || self.is_day_an_wu()
    }

    pub fn to_string_cn(&self) -> String {
        format!("{}年{}月{}", self.year_in_chinese(), self.month_in_chinese(), self.day_in_chinese())
    }

    #[cfg(feature = "i18n")]
    pub fn to_string_in_lang(&self, language: crate::i18n::Language) -> String {
        match language {
            crate::i18n::Language::ZhCn => self.to_string_cn(),
            crate::i18n::Language::En => format!("Taoist {}-{:02}-{:02}", self.year(), self.month().abs(), self.day()),
        }
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
        if matches!(language, crate::i18n::Language::ZhCn) {
            return self.to_full_string();
        }
        format!(
            "Taoist {} Year, TianYun {} Year, {} Month, {} Day. {} Month {} Day, {} Hour.",
            self.year(),
            self.lunar.year_in_gan_zhi_in_lang(language),
            self.lunar.month_in_gan_zhi_in_lang(language),
            self.lunar.day_in_gan_zhi_in_lang(language),
            self.month(),
            self.day(),
            self.lunar.time_in_gan_zhi_in_lang(language)
        )
    }
}
