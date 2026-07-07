//! 阳历日期 / 时间。对应 lunar-go `calendar/Solar.go`。

use std::fmt;

use crate::anno_lucis::AnnoLucis;
use crate::armenian::Armenian;
use crate::assyrian::Assyrian;
use crate::auc::Auc;
use crate::bengali::Bengali;
use crate::byzantine::Byzantine;
use crate::coptic::Coptic;
use crate::culture::{HideHeavenStem, HideHeavenStemDay, HideHeavenStemType, Nine, NineDay, SolarTermDay, Week};
use crate::dangi::Dangi;
use crate::ethiopian::Ethiopian;
use crate::event::{
    CalendarKind, Event, EventQuery, HolidayEvent, JieQiEvent, SolarFestivalEvent, all_events_for_day,
    find_events_for_day, holiday_period_events_for_day, scan_events_in_range, scan_events_in_range_filtered,
};
use crate::fasli::Fasli;
use crate::hijri::Hijri;
use crate::hispanic_era::HispanicEra;
use crate::holiday_util;
use crate::holocene::Holocene;
use crate::japanese::Japanese;
use crate::juche::Juche;
use crate::julian::Julian;
use crate::koki::Koki;
use crate::lunar::Lunar;
use crate::lunar_year::JIE_QI;
use crate::minguo::Minguo;
use crate::multi_calendar::CalendarDay;
use crate::nanakshahi::Nanakshahi;
use crate::rab_byung::{RabByungDay, RabByungYear};
use crate::rattanakosin::Rattanakosin;
use crate::rumi::Rumi;
use crate::saka::Saka;
use crate::seleucid::Seleucid;
use crate::solar_util;
use crate::thai_buddhist::ThaiBuddhist;
use crate::thai_solar::ThaiSolar;
use crate::venetian::Venetian;
use crate::{Constellation, JieQi, JulianDay, LunarError};

const HIDE_HEAVEN_STEM_DAY_DATA: &str = "93705542220504xx1513904541632524533533105544806564xx7573304542018584xx95";
const HIDE_HEAVEN_STEM_DAY_COUNTS: [usize; 6] = [3, 5, 7, 9, 10, 30];

/// 阳历日期时间。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Solar {
    pub(crate) year: i32,
    pub(crate) month: i32,
    pub(crate) day: i32,
    pub(crate) hour: i32,
    pub(crate) minute: i32,
    pub(crate) second: i32,
}

impl Solar {
    /// 由年月日时分秒构造（非法返回 `Err`）。
    pub fn from_ymd_hms(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
    ) -> Result<Self, LunarError> {
        if !(1..=12).contains(&month) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        if !(1..=31).contains(&day) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        if year == 1582 && month == 10 {
            if day > 4 && day < 15 {
                return Err(LunarError::GregorianGap { year, month, day });
            }
        } else if day > solar_util::days_of_month(year, month) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        if !(0..=23).contains(&hour) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        if !(0..=59).contains(&minute) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        if !(0..=59).contains(&second) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        Ok(Self { year, month, day, hour, minute, second })
    }

    /// 仅年月日（时分秒为 0）。
    #[inline]
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        Self::from_ymd_hms(year, month, day, 0, 0, 0)
    }

    /// 由儒略日反推阳历（时分秒通过小数部分恢复）。
    pub fn from_julian_day(julian_day: f64) -> Self {
        let mut d = (julian_day + 0.5) as i64;
        let mut f = julian_day + 0.5 - d as f64;

        if d >= 2_299_161 {
            let c = ((d as f64 - 1_867_216.25) / 36524.25) as i64;
            d += 1 + c - c / 4;
        }
        d += 1524;
        let mut year = ((d as f64 - 122.1) / 365.25) as i64;
        let mut d2 = d - (365.25 * year as f64) as i64;
        let mut month = (d2 as f64 / 30.601) as i64;
        d2 -= (30.601 * month as f64) as i64;
        let mut day = d2;
        if month > 13 {
            month -= 13;
            year -= 4715;
        } else {
            month -= 1;
            year -= 4716;
        }
        f *= 24.0;
        let mut hour = f as i64;
        f -= hour as f64;
        f *= 60.0;
        let mut minute = f as i64;
        f -= minute as f64;
        f *= 60.0;
        let mut second = f.round() as i64;

        if second > 59 {
            second -= 60;
            minute += 1;
        }
        if minute > 59 {
            minute -= 60;
            hour += 1;
        }
        if hour > 23 {
            hour -= 24;
            day += 1;
        }

        Self::from_ymd_hms(year as i32, month as i32, day as i32, hour as i32, minute as i32, second as i32).unwrap_or(
            Self {
                year: year as i32,
                month: month as i32,
                day: day as i32,
                hour: hour as i32,
                minute: minute as i32,
                second: second as i32,
            },
        )
    }

    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }
    #[inline]
    pub const fn get_year(&self) -> i32 {
        self.year()
    }
    #[inline]
    pub const fn month(&self) -> i32 {
        self.month
    }
    #[inline]
    pub const fn get_month(&self) -> i32 {
        self.month()
    }
    #[inline]
    pub const fn day(&self) -> i32 {
        self.day
    }
    #[inline]
    pub const fn get_day(&self) -> i32 {
        self.day()
    }
    #[inline]
    pub const fn hour(&self) -> i32 {
        self.hour
    }
    #[inline]
    pub const fn get_hour(&self) -> i32 {
        self.hour()
    }
    #[inline]
    pub const fn minute(&self) -> i32 {
        self.minute
    }
    #[inline]
    pub const fn get_minute(&self) -> i32 {
        self.minute()
    }
    #[inline]
    pub const fn second(&self) -> i32 {
        self.second
    }
    #[inline]
    pub const fn get_second(&self) -> i32 {
        self.second()
    }

    #[inline]
    pub const fn is_leap_year(&self) -> bool {
        solar_util::is_leap_year(self.year)
    }

    #[inline]
    pub fn week(&self) -> i32 {
        solar_util::week(self.year, self.month, self.day)
    }

    /// 星期几（中文）。
    #[inline]
    pub fn week_in_chinese(&self) -> &'static str {
        solar_util::WEEK[self.week() as usize]
    }

    /// 星期 typed 对象。
    #[inline]
    pub fn week_info(&self) -> Week {
        Week::from_index(self.week() as usize)
    }

    #[inline]
    pub fn get_week(&self) -> Week {
        self.week_info()
    }

    #[inline]
    pub fn get_solar_month(&self) -> crate::SolarMonth {
        crate::SolarMonth::from_ym(self.year, self.month)
    }

    /// 星期几（显式语言版本，需启用 `i18n` feature）。
    #[cfg(feature = "i18n")]
    pub fn week_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::week(self.week_in_chinese(), language)
    }

    /// 星座。
    pub fn xing_zuo(&self) -> &'static str {
        let m = self.month;
        let d = self.day;
        let y = m * 100 + d;
        let index = if (321..=419).contains(&y) {
            0
        } else if (420..=520).contains(&y) {
            1
        } else if (521..=621).contains(&y) {
            2
        } else if (622..=722).contains(&y) {
            3
        } else if (723..=822).contains(&y) {
            4
        } else if (823..=922).contains(&y) {
            5
        } else if (923..=1023).contains(&y) {
            6
        } else if (1024..=1122).contains(&y) {
            7
        } else if (1123..=1221).contains(&y) {
            8
        } else if y >= 1222 || y <= 119 {
            9
        } else if y <= 218 {
            10
        } else {
            11
        };
        solar_util::XINGZUO[index]
    }

    /// 星座 typed 对象。
    pub fn constellation(&self) -> Constellation {
        let name = self.xing_zuo();
        let index = solar_util::XINGZUO.iter().position(|value| *value == name).unwrap_or(0);
        Constellation::from_index(index)
    }

    pub fn get_constellation(&self) -> Constellation {
        self.constellation()
    }

    /// 星座（显式语言版本，需启用 `i18n` feature）。
    #[cfg(feature = "i18n")]
    pub fn xing_zuo_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::constellation(self.xing_zuo(), language)
    }

    /// 儒略日。
    #[inline]
    pub fn julian_day(&self) -> f64 {
        solar_util::julian_day(self.year, self.month, self.day, self.hour, self.minute, self.second)
    }

    #[inline]
    pub fn get_julian_day(&self) -> JulianDay {
        JulianDay::from_julian_day(self.julian_day())
    }

    #[inline]
    pub fn get_solar_time(&self) -> Self {
        *self
    }

    pub fn get_solar_day(&self) -> Self {
        Self::from_ymd(self.year, self.month, self.day).unwrap_or(*self)
    }

    pub fn get_solar_week(&self, start: i32) -> crate::SolarWeek {
        crate::SolarWeek::from_ymd(self.year, self.month, self.day, start)
    }

    pub fn get_index_in_year(&self) -> usize {
        let first_day = Self::from_ymd(self.year, 1, 1).unwrap_or(*self);
        self.subtract(&first_day) as usize
    }

    /// 转农历。
    pub fn lunar(&self) -> Lunar {
        Lunar::from_solar(*self)
    }

    #[inline]
    pub fn get_lunar_day(&self) -> Lunar {
        self.lunar()
    }

    pub fn get_lunar_hour(&self) -> crate::LunarHour<'static> {
        crate::LunarTime::from_ymd_hms(
            self.lunar().year(),
            self.lunar().month(),
            self.lunar().day(),
            self.hour,
            self.minute,
            self.second,
        )
        .expect("solar-derived lunar hour must be valid")
    }

    pub fn get_term(&self) -> JieQi {
        let lunar = self.lunar();
        let mut term =
            lunar.current_jie_qi().or_else(|| lunar.prev_jie_qi()).unwrap_or_else(|| JieQi::from_index(self.year, 0));
        if self.julian_day() < term.solar().julian_day() {
            term = term.next(-1);
        }
        term
    }

    pub fn get_term_day(&self) -> Option<SolarTermDay> {
        self.lunar().solar_term_day()
    }

    pub fn get_phenology_day(&self) -> Option<crate::PhenologyDay> {
        self.lunar().phenology_day()
    }

    pub fn get_phenology(&self) -> crate::Phenology {
        self.lunar().phenology()
    }

    /// 转回历（公历民用回历）。
    pub fn hijri(&self) -> Hijri {
        Hijri::from_solar(*self)
    }

    pub fn get_hijri_day(&self) -> crate::HijriDay {
        self.hijri()
    }

    pub fn get_sixty_cycle_hour(&self) -> crate::SixtyCycleHour {
        self.lunar().sixty_cycle_hour()
    }

    pub fn hijri_year(&self) -> crate::HijriYear {
        self.hijri().hijri_year()
    }

    pub fn hijri_month(&self) -> crate::HijriMonth {
        self.hijri().hijri_month()
    }

    /// 转民国历。
    pub fn minguo(&self) -> Minguo {
        Minguo::from_solar(*self)
    }

    pub fn minguo_year(&self) -> crate::MinguoYear {
        self.minguo().minguo_year()
    }

    pub fn minguo_month(&self) -> crate::MinguoMonth {
        self.minguo().minguo_month()
    }

    /// 转泰阳历（泰佛历公历纪年）。
    pub fn thai_solar(&self) -> ThaiSolar {
        ThaiSolar::from_solar(*self)
    }

    pub fn thai_solar_year(&self) -> crate::ThaiSolarYear {
        self.thai_solar().thai_solar_year()
    }

    pub fn thai_solar_month(&self) -> crate::ThaiSolarMonth {
        self.thai_solar().thai_solar_month()
    }

    /// 转现代日本年号历。
    pub fn japanese(&self) -> Result<Japanese, LunarError> {
        Japanese::from_solar(*self)
    }

    pub fn japanese_year(&self) -> Result<crate::JapaneseYear, LunarError> {
        self.japanese().map(|date| date.japanese_year())
    }

    pub fn japanese_month(&self) -> Result<crate::JapaneseMonth, LunarError> {
        self.japanese().map(|date| date.japanese_month())
    }

    /// 转主体纪年。
    pub fn juche(&self) -> Result<Juche, LunarError> {
        Juche::from_solar(*self)
    }

    pub fn juche_year(&self) -> Result<crate::JucheYear, LunarError> {
        self.juche().map(|date| date.juche_year())
    }

    pub fn juche_month(&self) -> Result<crate::JucheMonth, LunarError> {
        self.juche().map(|date| date.juche_month())
    }

    /// 转檀纪。
    pub fn dangi(&self) -> Result<Dangi, LunarError> {
        Dangi::from_solar(*self)
    }

    pub fn dangi_year(&self) -> Result<crate::DangiYear, LunarError> {
        self.dangi().map(|date| date.dangi_year())
    }

    pub fn dangi_month(&self) -> Result<crate::DangiMonth, LunarError> {
        self.dangi().map(|date| date.dangi_month())
    }

    /// 转儒略历。
    pub fn julian_calendar(&self) -> Julian {
        Julian::from_solar(*self)
    }

    pub fn julian_calendar_year(&self) -> crate::JulianYear {
        self.julian_calendar().julian_year()
    }

    pub fn julian_calendar_month(&self) -> crate::JulianMonth {
        self.julian_calendar().julian_month()
    }

    /// 转全新世纪年。
    pub fn holocene(&self) -> Result<Holocene, LunarError> {
        Holocene::from_solar(*self)
    }

    pub fn holocene_year(&self) -> Result<crate::HoloceneYear, LunarError> {
        self.holocene().map(|date| date.holocene_year())
    }

    pub fn holocene_month(&self) -> Result<crate::HoloceneMonth, LunarError> {
        self.holocene().map(|date| date.holocene_month())
    }

    /// 转拜占庭纪年。
    pub fn byzantine(&self) -> Result<Byzantine, LunarError> {
        Ok(Byzantine::from_solar(*self))
    }

    pub fn byzantine_year(&self) -> Result<crate::ByzantineYear, LunarError> {
        self.byzantine().map(|date| date.byzantine_year())
    }

    pub fn byzantine_month(&self) -> Result<crate::ByzantineMonth, LunarError> {
        self.byzantine().map(|date| date.byzantine_month())
    }

    /// 转科普特历。
    pub fn coptic(&self) -> Coptic {
        Coptic::from_solar(*self)
    }

    pub fn coptic_year(&self) -> crate::CopticYear {
        self.coptic().coptic_year()
    }

    pub fn coptic_month(&self) -> crate::CopticMonth {
        self.coptic().coptic_month()
    }

    /// 转亚美尼亚历。
    pub fn armenian(&self) -> Armenian {
        Armenian::from_solar(*self)
    }

    pub fn armenian_year(&self) -> crate::ArmenianYear {
        self.armenian().armenian_year()
    }

    pub fn armenian_month(&self) -> crate::ArmenianMonth {
        self.armenian().armenian_month()
    }

    /// 转光明纪年。
    pub fn anno_lucis(&self) -> Result<AnnoLucis, LunarError> {
        AnnoLucis::from_solar(*self)
    }

    pub fn anno_lucis_year(&self) -> Result<crate::AnnoLucisYear, LunarError> {
        self.anno_lucis().map(|date| date.anno_lucis_year())
    }

    pub fn anno_lucis_month(&self) -> Result<crate::AnnoLucisMonth, LunarError> {
        self.anno_lucis().map(|date| date.anno_lucis_month())
    }

    /// 转罗马建城纪年。
    pub fn auc(&self) -> Result<Auc, LunarError> {
        Auc::from_solar(*self)
    }

    pub fn auc_year(&self) -> Result<crate::AucYear, LunarError> {
        self.auc().map(|date| date.auc_year())
    }

    pub fn auc_month(&self) -> Result<crate::AucMonth, LunarError> {
        self.auc().map(|date| date.auc_month())
    }

    /// 转亚述纪年。
    pub fn assyrian(&self) -> Result<Assyrian, LunarError> {
        Assyrian::from_solar(*self)
    }

    pub fn assyrian_year(&self) -> Result<crate::AssyrianYear, LunarError> {
        self.assyrian().map(|date| date.assyrian_year())
    }

    pub fn assyrian_month(&self) -> Result<crate::AssyrianMonth, LunarError> {
        self.assyrian().map(|date| date.assyrian_month())
    }

    /// 转西班牙纪元。
    pub fn hispanic_era(&self) -> Result<HispanicEra, LunarError> {
        HispanicEra::from_solar(*self)
    }

    pub fn hispanic_era_year(&self) -> Result<crate::HispanicEraYear, LunarError> {
        self.hispanic_era().map(|date| date.hispanic_era_year())
    }

    pub fn hispanic_era_month(&self) -> Result<crate::HispanicEraMonth, LunarError> {
        self.hispanic_era().map(|date| date.hispanic_era_month())
    }

    /// 转萨卡历（印度国历）。
    pub fn saka(&self) -> Result<Saka, LunarError> {
        Saka::from_solar(*self)
    }

    pub fn saka_year(&self) -> Result<crate::SakaYear, LunarError> {
        self.saka().map(|date| date.saka_year())
    }

    pub fn saka_month(&self) -> Result<crate::SakaMonth, LunarError> {
        self.saka().map(|date| date.saka_month())
    }

    /// 转孟加拉历。
    pub fn bengali(&self) -> Result<Bengali, LunarError> {
        Bengali::from_solar(*self)
    }

    pub fn bengali_year(&self) -> Result<crate::BengaliYear, LunarError> {
        self.bengali().map(|date| date.bengali_year())
    }

    pub fn bengali_month(&self) -> Result<crate::BengaliMonth, LunarError> {
        self.bengali().map(|date| date.bengali_month())
    }

    /// 转皇纪。
    pub fn koki(&self) -> Result<Koki, LunarError> {
        Koki::from_solar(*self)
    }

    pub fn koki_year(&self) -> Result<crate::KokiYear, LunarError> {
        self.koki().map(|date| date.koki_year())
    }

    pub fn koki_month(&self) -> Result<crate::KokiMonth, LunarError> {
        self.koki().map(|date| date.koki_month())
    }

    /// 转旧制泰佛历。
    pub fn thai_buddhist(&self) -> Result<ThaiBuddhist, LunarError> {
        ThaiBuddhist::from_solar(*self)
    }

    pub fn thai_buddhist_year(&self) -> Result<crate::ThaiBuddhistYear, LunarError> {
        self.thai_buddhist().map(|date| date.thai_buddhist_year())
    }

    pub fn thai_buddhist_month(&self) -> Result<crate::ThaiBuddhistMonth, LunarError> {
        self.thai_buddhist().map(|date| date.thai_buddhist_month())
    }

    /// 转法斯里历。
    pub fn fasli(&self) -> Result<Fasli, LunarError> {
        Fasli::from_solar(*self)
    }

    pub fn fasli_year(&self) -> Result<crate::FasliYear, LunarError> {
        self.fasli().map(|date| date.fasli_year())
    }

    pub fn fasli_month(&self) -> Result<crate::FasliMonth, LunarError> {
        self.fasli().map(|date| date.fasli_month())
    }

    /// 转纳纳克沙希历。
    pub fn nanakshahi(&self) -> Result<Nanakshahi, LunarError> {
        Nanakshahi::from_solar(*self)
    }

    pub fn nanakshahi_year(&self) -> Result<crate::NanakshahiYear, LunarError> {
        self.nanakshahi().map(|date| date.nanakshahi_year())
    }

    pub fn nanakshahi_month(&self) -> Result<crate::NanakshahiMonth, LunarError> {
        self.nanakshahi().map(|date| date.nanakshahi_month())
    }

    /// 转拉达那哥欣纪元。
    pub fn rattanakosin(&self) -> Result<Rattanakosin, LunarError> {
        Rattanakosin::from_solar(*self)
    }

    pub fn rattanakosin_year(&self) -> Result<crate::RattanakosinYear, LunarError> {
        self.rattanakosin().map(|date| date.rattanakosin_year())
    }

    pub fn rattanakosin_month(&self) -> Result<crate::RattanakosinMonth, LunarError> {
        self.rattanakosin().map(|date| date.rattanakosin_month())
    }

    /// 转塞琉古纪元。
    pub fn seleucid(&self) -> Result<Seleucid, LunarError> {
        Seleucid::from_solar(*self)
    }

    pub fn seleucid_year(&self) -> Result<crate::SeleucidYear, LunarError> {
        self.seleucid().map(|date| date.seleucid_year())
    }

    pub fn seleucid_month(&self) -> Result<crate::SeleucidMonth, LunarError> {
        self.seleucid().map(|date| date.seleucid_month())
    }

    /// 转埃塞俄比亚历。
    pub fn ethiopian(&self) -> Ethiopian {
        Ethiopian::from_solar(*self)
    }

    pub fn ethiopian_year(&self) -> crate::EthiopianYear {
        self.ethiopian().ethiopian_year()
    }

    pub fn ethiopian_month(&self) -> crate::EthiopianMonth {
        self.ethiopian().ethiopian_month()
    }

    /// 转威尼斯纪年（More Veneto）。
    pub fn venetian(&self) -> Result<Venetian, LunarError> {
        Venetian::from_solar(*self)
    }

    pub fn venetian_year(&self) -> Result<crate::VenetianYear, LunarError> {
        self.venetian().map(|date| date.venetian_year())
    }

    pub fn venetian_month(&self) -> Result<crate::VenetianMonth, LunarError> {
        self.venetian().map(|date| date.venetian_month())
    }

    /// 转鲁米历。
    pub fn rumi(&self) -> Result<Rumi, LunarError> {
        Rumi::from_solar(*self)
    }

    pub fn rumi_year(&self) -> Result<crate::RumiYear, LunarError> {
        self.rumi().map(|date| date.rumi_year())
    }

    pub fn rumi_month(&self) -> Result<crate::RumiMonth, LunarError> {
        self.rumi().map(|date| date.rumi_month())
    }

    /// 转藏历年（饶迥年）。
    pub fn rab_byung_year(&self) -> Result<RabByungYear, LunarError> {
        RabByungYear::from_year(self.year)
    }

    /// 转藏历日（饶迥历日）。
    pub fn rab_byung_day(&self) -> Result<RabByungDay, LunarError> {
        RabByungDay::from_solar(*self)
    }

    pub fn rab_byung_month(&self) -> Result<crate::RabByungMonth, LunarError> {
        self.rab_byung_day().map(|day| day.rab_byung_month())
    }

    /// `YYYY-MM-DD`。
    pub fn to_ymd(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// `YYYY-MM-DD HH:MM:SS`。
    pub fn to_ymd_hms(&self) -> String {
        format!("{} {:02}:{:02}:{:02}", self.to_ymd(), self.hour, self.minute, self.second)
    }

    /// 显式语言版本的基础字符串。
    #[cfg(feature = "i18n")]
    pub fn to_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_solar_string(self)
    }

    /// 完整字符串：日期 + 闰年 + 星期 + 节日 + 星座。
    pub fn to_full_string(&self) -> String {
        let mut s = self.to_ymd_hms();
        if self.is_leap_year() {
            s += " 闰年";
        }
        s += " 星期";
        s += self.week_in_chinese();
        for f in self.festivals() {
            s += " (";
            s += f;
            s += ")";
        }
        for f in self.other_festivals() {
            s += " (";
            s += f;
            s += ")";
        }
        s.push(' ');
        s += self.xing_zuo();
        s += "座";
        s
    }

    /// 完整字符串（显式语言版本，需启用 `i18n` feature）。
    #[cfg(feature = "i18n")]
    pub fn to_full_string_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::locale(language).render_solar_full(self)
    }

    /// 节日（几月几日 + 第 N 个星期几）。
    pub fn festivals(&self) -> Vec<&'static str> {
        let mut l = Vec::new();
        if let Some(festival) = solar_util::festival(self.month, self.day) {
            l.push(festival);
        }
        let weeks = (f64::from(self.day) / 7.0).ceil() as i32;
        let week = self.week();
        if let Some(festival) = solar_util::week_festival(self.month, weeks, week) {
            l.push(festival);
        }
        if self.day + 7 > solar_util::days_of_month(self.year, self.month) {
            if let Some(festival) = solar_util::week_festival(self.month, 0, week) {
                l.push(festival);
            }
        }
        l
    }

    /// 其它节日。
    pub fn other_festivals(&self) -> Vec<&'static str> {
        solar_util::other_festivals(self.month, self.day).to_vec()
    }

    pub fn nine_day(&self) -> Option<NineDay> {
        let shu_jiu = self.lunar().shu_jiu()?;
        let nine = Nine::from_name(shu_jiu.name())?;
        Some(NineDay::new(nine, shu_jiu.day_index()))
    }

    pub fn get_nine_day(&self) -> Option<NineDay> {
        self.nine_day()
    }

    pub fn hide_heaven_stem_day(&self) -> Option<HideHeavenStemDay> {
        let lunar = self.lunar();
        let current_jie = lunar.jie();
        let term = if current_jie.is_empty() { lunar.prev_jie_by_whole_day(true)? } else { lunar.current_jie_qi()? };
        let term_index = JIE_QI.iter().position(|name| *name == term.name())?;
        let start_index = term_index.checked_sub(1)? * 3;
        let data = HIDE_HEAVEN_STEM_DAY_DATA.get(start_index..start_index + 6)?;
        let term_solar = term.solar();
        let term_day = Solar::from_ymd(term_solar.year(), term_solar.month(), term_solar.day()).ok()?;
        let current_day = Solar::from_ymd(self.year, self.month, self.day).ok()?;
        let mut day_index = current_day.subtract(&term_day) as usize;
        let mut days = 0_usize;
        let mut heaven_stem_index = 0_usize;
        let mut kind_index = 0_usize;

        while kind_index < 3 {
            let index = kind_index * 2;
            let marker = data.get(index..index + 1)?;
            let mut count = 0_usize;
            if marker != "x" {
                heaven_stem_index = marker.parse().ok()?;
                let count_index: usize = data.get(index + 1..index + 2)?.parse().ok()?;
                count = HIDE_HEAVEN_STEM_DAY_COUNTS[count_index];
                days += count;
            }
            if day_index <= days {
                day_index -= days - count;
                break;
            }
            kind_index += 1;
        }

        let kind = HideHeavenStemType::from_index(kind_index)?;
        Some(HideHeavenStemDay::new(HideHeavenStem::from_index(heaven_stem_index, kind), day_index as i32 + 1))
    }

    pub fn get_hide_heaven_stem_day(&self) -> Option<HideHeavenStemDay> {
        self.hide_heaven_stem_day()
    }

    pub fn get_dog_day(&self) -> Option<crate::DogDay> {
        self.lunar().dog_day()
    }

    pub fn get_plum_rain_day(&self) -> Option<crate::PlumRainDay> {
        self.lunar().plum_rain_day()
    }

    pub fn get_legal_holiday(&self) -> Option<crate::Holiday> {
        crate::Holiday::from_ymd(self.year, self.month, self.day)
    }

    pub fn get_sixty_cycle_day(&self) -> crate::SixtyCycleDay {
        self.lunar().sixty_cycle_day()
    }

    pub fn get_phase_day(&self) -> crate::PhaseDay {
        self.lunar().phase_day()
    }

    pub fn get_phase(&self) -> crate::Phase {
        self.lunar().phase()
    }

    pub fn get_nine_star(&self) -> crate::NineStar {
        self.lunar().day_nine_star()
    }

    pub fn get_rab_byung_day(&self) -> Result<RabByungDay, LunarError> {
        self.rab_byung_day()
    }

    pub fn get_festival(&self) -> Option<crate::SolarFestival> {
        crate::SolarFestival::from_ymd(self.year, self.month, self.day)
    }

    /// Unified events for the current solar date.
    pub fn events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        for name in self.festivals() {
            events.push(SolarFestivalEvent::new(*self, name, false).to_event());
        }
        for name in self.other_festivals() {
            events.push(SolarFestivalEvent::new(*self, name, true).to_event());
        }
        for holiday in holiday_util::get_holidays_by_ymd(self.year, self.month, self.day) {
            events.push(HolidayEvent::new(holiday, *self, CalendarKind::Solar).to_event());
        }
        if let Some(jieqi) = self.lunar().current_jie_qi() {
            events.push(JieQiEvent::new(jieqi, CalendarKind::Solar).to_event());
        }
        events.extend(holiday_period_events_for_day(*self));

        events
    }

    /// Aggregated events across solar, lunar, buddhist and taoist contexts.
    pub fn all_events(&self) -> Vec<Event> {
        all_events_for_day(*self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        find_events_for_day(*self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        scan_events_in_range(*self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        scan_events_in_range_filtered(*self, end, query)
    }

    /// 与另一日期的天数差（self - other）。
    pub fn subtract(&self, other: &Self) -> i32 {
        solar_util::days_between(other.year, other.month, other.day, self.year, self.month, self.day)
    }

    /// 与另一时刻的分钟差（self - other）。
    pub fn subtract_minute(&self, other: &Self) -> i32 {
        let mut days = self.subtract(other);
        let cm = self.hour * 60 + self.minute;
        let sm = other.hour * 60 + other.minute;
        let mut m = cm - sm;
        if m < 0 {
            m += 1440;
            days -= 1;
        }
        m + days * 1440
    }

    pub fn is_after(&self, other: &Self) -> bool {
        (self.year, self.month, self.day, self.hour, self.minute, self.second)
            > (other.year, other.month, other.day, other.hour, other.minute, other.second)
    }

    pub fn is_before(&self, other: &Self) -> bool {
        solar_util::is_before(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            other.year,
            other.month,
            other.day,
            other.hour,
            other.minute,
            other.second,
        )
    }

    /// 推进 / 回退若干年。
    pub fn next_year(&self, years: i32) -> Self {
        let y = self.year + years;
        let m = self.month;
        let mut d = self.day;
        if y == 1582 && m == 10 {
            if d > 4 && d < 15 {
                d += 10;
            }
        } else if m == 2 && d > 28 && !solar_util::is_leap_year(y) {
            d = 28;
        }
        Self::from_ymd_hms(y, m, d, self.hour, self.minute, self.second).unwrap_or(*self)
    }

    /// 推进 / 回退若干月。
    pub fn next_month(&self, months: i32) -> Self {
        let (y, m) = next_ym(self.year, self.month, months);
        let mut d = self.day;
        if y == 1582 && m == 10 {
            if d > 4 && d < 15 {
                d += 10;
            }
        } else {
            let max_day = solar_util::days_of_month(y, m);
            if d > max_day {
                d = max_day;
            }
        }
        Self::from_ymd_hms(y, m, d, self.hour, self.minute, self.second).unwrap_or(*self)
    }

    /// 推进 / 回退若干天（正确处理 1582 历法改革跳过的 10 天）。
    pub fn next_day(&self, days: i32) -> Self {
        let mut y = self.year;
        let mut m = self.month;
        let mut d = self.day;
        if y == 1582 && m == 10 && d > 4 {
            d -= 10;
        }
        if days > 0 {
            d += days;
            let mut days_in_month = solar_util::days_of_month(y, m);
            while d > days_in_month {
                d -= days_in_month;
                m += 1;
                if m > 12 {
                    m = 1;
                    y += 1;
                }
                days_in_month = solar_util::days_of_month(y, m);
            }
        } else if days < 0 {
            while d + days <= 0 {
                m -= 1;
                if m < 1 {
                    m = 12;
                    y -= 1;
                }
                d += solar_util::days_of_month(y, m);
            }
            d += days;
        }
        if y == 1582 && m == 10 && d > 4 {
            d += 10;
        }
        Self::from_ymd_hms(y, m, d, self.hour, self.minute, self.second).unwrap_or(*self)
    }

    /// 推进若干天，可选仅工作日。
    pub fn next(&self, days: i32, only_workday: bool) -> Self {
        if !only_workday {
            return self.next_day(days);
        }
        let mut o = *self;
        if days != 0 {
            let (mut rest, add) = if days < 0 { (-days, -1) } else { (days, 1) };
            while rest > 0 {
                o = o.next_day(add);
                let work = holiday_util::get_holiday_by_ymd(o.year, o.month, o.day).map_or_else(
                    || {
                        let w = o.week();
                        !(w == 0 || w == 6)
                    },
                    |holiday| holiday.is_work(),
                );
                if work {
                    rest -= 1;
                }
            }
        }
        o
    }

    /// 推进 / 回退若干小时。
    pub fn next_hour(&self, hours: i32) -> Self {
        let h = self.hour + hours;
        let (n, mut hour) = if h < 0 { (-1, -h) } else { (1, h) };
        let mut days = hour / 24 * n;
        hour = hour % 24 * n;
        if hour < 0 {
            hour += 24;
            days -= 1;
        }
        let o = self.next_day(days);
        Self::from_ymd_hms(o.year, o.month, o.day, hour, o.minute, o.second).unwrap_or(o)
    }

    /// 薪资倍率（1 平时 / 2 休息日 / 3 法定节假日）。
    pub fn salary_rate(&self) -> u8 {
        if self.month == 1 && self.day == 1 {
            return 3;
        }
        if self.month == 5 && self.day == 1 {
            return 3;
        }
        if self.month == 10 && (1..=3).contains(&self.day) {
            return 3;
        }
        let lunar = self.lunar();
        if lunar.month() == 1 && (1..=3).contains(&lunar.day()) {
            return 3;
        }
        if lunar.month() == 5 && lunar.day() == 5 {
            return 3;
        }
        if lunar.month() == 8 && lunar.day() == 15 {
            return 3;
        }
        if lunar.jie_qi() == "清明" {
            return 3;
        }
        if let Some(holiday) = holiday_util::get_holiday_by_ymd(self.year, self.month, self.day) {
            if !holiday.is_work() {
                return 2;
            }
        } else {
            let w = self.week();
            if w == 6 || w == 0 {
                return 2;
            }
        }
        1
    }
}

impl fmt::Display for Solar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ymd())
    }
}

impl CalendarDay for Solar {
    fn solar(&self) -> Solar {
        *self
    }
}

/// 计算 (year, month) + months 后的 (year, month)。
pub const fn next_ym(year: i32, month: i32, months: i32) -> (i32, i32) {
    let total = (year * 12 + (month - 1)) + months;
    let y = total.div_euclid(12);
    let m = total.rem_euclid(12) + 1;
    (y, m)
}
