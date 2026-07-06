//! 农历（核心 god-class）。对应 lunar-go `calendar/Lunar.go`。

use std::collections::HashMap;
use std::fmt;

use crate::LunarError;
use crate::eight_char::EightChar;
use crate::fu::Fu;
use crate::jieqi::JieQi;
use crate::lunar_time::LunarTime;
use crate::lunar_util;
use crate::lunar_year::{JIE_QI, JIE_QI_IN_USE, LunarYear};
use crate::nine_star::NineStar;
use crate::shu_jiu::ShuJiu;
use crate::solar::Solar;
use crate::solar_util;

/// 农历日期时间。
pub struct Lunar {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    year_gan_index: i64,
    year_zhi_index: i64,
    year_gan_index_by_li_chun: i64,
    year_zhi_index_by_li_chun: i64,
    year_gan_index_exact: i64,
    year_zhi_index_exact: i64,
    month_gan_index: i64,
    month_zhi_index: i64,
    month_gan_index_exact: i64,
    month_zhi_index_exact: i64,
    day_gan_index: i64,
    day_zhi_index: i64,
    day_gan_index_exact: i64,
    day_zhi_index_exact: i64,
    day_gan_index_exact2: i64,
    day_zhi_index_exact2: i64,
    time_gan_index: i64,
    time_zhi_index: i64,
    week_index: i32,
    jie_qi: HashMap<&'static str, Solar>,
    solar: Solar,
}

fn convert_jie_qi(name: &str) -> &'static str {
    match name {
        "DONG_ZHI" => "冬至",
        "DA_HAN" => "大寒",
        "XIAO_HAN" => "小寒",
        "LI_CHUN" => "立春",
        "DA_XUE" => "大雪",
        "YU_SHUI" => "雨水",
        "JING_ZHE" => "惊蛰",
        other => {
            // JIE_QI_IN_USE 的中文项本身是 &'static str；直接转。
            // 这里 other 必为表中静态字符串之一。
            static_all_jieqi(other)
        }
    }
}

// 在 JIE_QI_IN_USE 中查找静态字符串（中文项），保证返回 'static。
fn static_all_jieqi(name: &str) -> &'static str {
    for s in JIE_QI_IN_USE.iter() {
        if *s == name {
            return *s;
        }
    }
    // 兜底（不应到达）
    "冬至"
}

impl Lunar {
    /// 由农历年月日时分秒构造。
    pub fn from_ymd_hms(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
    ) -> Result<Self, LunarError> {
        let y = LunarYear::from_year(year);
        let m = y.get_month(month).ok_or(LunarError::LeapMonthAbsent { year, month })?;
        if day < 1 {
            return Err(LunarError::InvalidLunar { year, month, day });
        }
        if day > m.day_count() {
            return Err(LunarError::LunarDayOverflow { year, month, day, max: m.day_count() });
        }
        let noon = Solar::from_julian_day(m.first_julian_day() + (day - 1) as f64);
        let solar = Solar::from_ymd_hms(noon.year(), noon.month(), noon.day(), hour, minute, second)
            .map_err(|_| LunarError::InvalidLunar { year, month, day })?;
        let lunar_year = if noon.year() != year { LunarYear::from_year(noon.year()) } else { y };
        let mut lunar = Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            year_gan_index: 0,
            year_zhi_index: 0,
            year_gan_index_by_li_chun: 0,
            year_zhi_index_by_li_chun: 0,
            year_gan_index_exact: 0,
            year_zhi_index_exact: 0,
            month_gan_index: 0,
            month_zhi_index: 0,
            month_gan_index_exact: 0,
            month_zhi_index_exact: 0,
            day_gan_index: 0,
            day_zhi_index: 0,
            day_gan_index_exact: 0,
            day_zhi_index_exact: 0,
            day_gan_index_exact2: 0,
            day_zhi_index_exact2: 0,
            time_gan_index: 0,
            time_zhi_index: 0,
            week_index: 0,
            jie_qi: HashMap::new(),
            solar,
        };
        lunar.compute(&lunar_year);
        Ok(lunar)
    }

    /// 仅年月日。
    #[inline]
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        Self::from_ymd_hms(year, month, day, 0, 0, 0)
    }

    /// 由阳历构造。
    pub fn from_solar(solar: Solar) -> Self {
        let ly = LunarYear::from_year(solar.year());
        let mut lunar_year = 0_i32;
        let mut lunar_month = 0_i32;
        let mut lunar_day = 0_i32;
        for m in ly.months().iter() {
            let days = solar.subtract(&Solar::from_julian_day(m.first_julian_day()));
            if days < m.day_count() {
                lunar_year = m.year();
                lunar_month = m.month();
                lunar_day = days + 1;
                break;
            }
        }
        let mut lunar = Self {
            year: lunar_year,
            month: lunar_month,
            day: lunar_day,
            hour: solar.hour(),
            minute: solar.minute(),
            second: solar.second(),
            year_gan_index: 0,
            year_zhi_index: 0,
            year_gan_index_by_li_chun: 0,
            year_zhi_index_by_li_chun: 0,
            year_gan_index_exact: 0,
            year_zhi_index_exact: 0,
            month_gan_index: 0,
            month_zhi_index: 0,
            month_gan_index_exact: 0,
            month_zhi_index_exact: 0,
            day_gan_index: 0,
            day_zhi_index: 0,
            day_gan_index_exact: 0,
            day_zhi_index_exact: 0,
            day_gan_index_exact2: 0,
            day_zhi_index_exact2: 0,
            time_gan_index: 0,
            time_zhi_index: 0,
            week_index: 0,
            jie_qi: HashMap::new(),
            solar,
        };
        lunar.compute(&ly);
        lunar
    }

    fn compute_jie_qi(&mut self, lunar_year: &LunarYear) {
        let julian_days = lunar_year.jie_qi_julian_days();
        let size = JIE_QI_IN_USE.len();
        for i in 0..size {
            let name = JIE_QI_IN_USE[i];
            self.jie_qi.insert(name, Solar::from_julian_day(julian_days[i]));
        }
    }

    fn compute_year(&mut self) {
        let offset = self.year - 4;
        let mut year_gan_index = offset.rem_euclid(10) as i64;
        let mut year_zhi_index = offset.rem_euclid(12) as i64;
        if year_gan_index < 0 {
            year_gan_index += 10;
        }
        if year_zhi_index < 0 {
            year_zhi_index += 12;
        }
        self.year_gan_index = year_gan_index;
        self.year_zhi_index = year_zhi_index;

        let mut g = year_gan_index;
        let mut z = year_zhi_index;
        let mut g_exact = year_gan_index;
        let mut z_exact = year_zhi_index;

        let solar_year = self.solar.year();
        let solar_ymd = self.solar.to_ymd();
        let solar_ymd_hms = self.solar.to_ymd_hms();

        let li_chun_solar = self.jq("立春");
        let li_chun = if li_chun_solar.year() != solar_year { self.jq("LI_CHUN") } else { li_chun_solar };
        let li_chun_ymd = li_chun.to_ymd();
        let li_chun_ymd_hms = li_chun.to_ymd_hms();

        if self.year == solar_year {
            if solar_ymd < li_chun_ymd {
                g -= 1;
                z -= 1;
            }
            if solar_ymd_hms < li_chun_ymd_hms {
                g_exact -= 1;
                z_exact -= 1;
            }
        } else if self.year < solar_year {
            if solar_ymd >= li_chun_ymd {
                g += 1;
                z += 1;
            }
            if solar_ymd_hms >= li_chun_ymd_hms {
                g_exact += 1;
                z_exact += 1;
            }
        }

        if g < 0 {
            g += 10;
        }
        if z < 0 {
            z += 12;
        }
        if g_exact < 0 {
            g_exact += 10;
        }
        if z_exact < 0 {
            z_exact += 12;
        }
        self.year_gan_index_by_li_chun = g % 10;
        self.year_zhi_index_by_li_chun = z % 12;
        self.year_gan_index_exact = g_exact % 10;
        self.year_zhi_index_exact = z_exact % 12;
    }

    fn compute_month(&mut self) {
        let ymd = self.solar.to_ymd();
        let ymdhms = self.solar.to_ymd_hms();
        let size = JIE_QI_IN_USE.len();

        let mut start: Option<Solar> = None;
        let mut index: i64 = -3;
        let mut i = 0;
        while i < size {
            let jie = JIE_QI_IN_USE[i];
            let end = self.jq(jie);
            let symd = match start {
                Some(s) => s.to_ymd(),
                None => ymd.clone(),
            };
            if ymd >= symd && ymd < end.to_ymd() {
                break;
            }
            start = Some(end);
            index += 1;
            i += 2;
        }
        let mut add = if index < 0 { 1 } else { 0 };
        let offset = (((self.year_gan_index_by_li_chun + add) % 5 + 1) * 2) % 10;
        add = index;
        if add < 0 {
            add += 10;
        }
        self.month_gan_index = (add + offset) % 10;
        add = index;
        if add < 0 {
            add += 12;
        }
        self.month_zhi_index = (add + lunar_util::BASE_MONTH_ZHI_INDEX) % 12;

        start = None;
        index = -3;
        i = 0;
        while i < size {
            let jie = JIE_QI_IN_USE[i];
            let end = self.jq(jie);
            let stime = match start {
                Some(s) => s.to_ymd_hms(),
                None => ymdhms.clone(),
            };
            if ymdhms >= stime && ymdhms < end.to_ymd_hms() {
                break;
            }
            start = Some(end);
            index += 1;
            i += 2;
        }
        add = if index < 0 { 1 } else { 0 };
        let offset = (((self.year_gan_index_exact + add) % 5 + 1) * 2) % 10;
        add = index;
        if add < 0 {
            add += 10;
        }
        self.month_gan_index_exact = (add + offset) % 10;
        add = index;
        if add < 0 {
            add += 12;
        }
        self.month_zhi_index_exact = (add + lunar_util::BASE_MONTH_ZHI_INDEX) % 12;
    }

    fn compute_day(&mut self) {
        let noon = Solar::from_ymd_hms(self.solar.year(), self.solar.month(), self.solar.day(), 12, 0, 0)
            .unwrap_or(self.solar);
        let offset = (noon.julian_day() - 11.0) as i64;
        self.day_gan_index = offset % 10;
        self.day_zhi_index = offset % 12;
        let mut day_gan_exact = self.day_gan_index;
        let mut day_zhi_exact = self.day_zhi_index;
        self.day_gan_index_exact2 = day_gan_exact;
        self.day_zhi_index_exact2 = day_zhi_exact;
        let hm = format!("{:02}:{:02}", self.hour, self.minute);
        if hm.as_str() >= "23:00" && hm.as_str() <= "23:59" {
            day_gan_exact += 1;
            if day_gan_exact >= 10 {
                day_gan_exact -= 10;
            }
            day_zhi_exact += 1;
            if day_zhi_exact >= 12 {
                day_zhi_exact -= 12;
            }
        }
        self.day_gan_index_exact = day_gan_exact;
        self.day_zhi_index_exact = day_zhi_exact;
    }

    fn compute_time(&mut self) {
        let hm = format!("{:02}:{:02}", self.hour, self.minute);
        self.time_zhi_index = lunar_util::get_time_zhi_index(&hm);
        self.time_gan_index = (self.day_gan_index_exact % 5 * 2 + self.time_zhi_index) % 10;
    }

    fn compute(&mut self, lunar_year: &LunarYear) {
        self.compute_jie_qi(lunar_year);
        self.compute_year();
        self.compute_month();
        self.compute_day();
        self.compute_time();
        self.week_index = self.solar.week();
    }

    fn jq(&self, key: &str) -> Solar {
        *self.jie_qi.get(key).unwrap_or(&Solar::from_ymd(2000, 1, 1).unwrap())
    }

    // ---- 字段访问 ----
    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }
    #[inline]
    pub const fn month(&self) -> i32 {
        self.month
    }
    #[inline]
    pub const fn day(&self) -> i32 {
        self.day
    }
    #[inline]
    pub const fn hour(&self) -> i32 {
        self.hour
    }
    #[inline]
    pub const fn minute(&self) -> i32 {
        self.minute
    }
    #[inline]
    pub const fn second(&self) -> i32 {
        self.second
    }
    #[inline]
    pub const fn solar(&self) -> Solar {
        self.solar
    }
    /// 往后 / 往前推 n 天。
    pub fn next(&self, days: i32) -> Lunar {
        self.solar.next_day(days).lunar()
    }

    // ---- 索引 ----
    pub const fn year_gan_index(&self) -> i64 {
        self.year_gan_index
    }
    pub const fn year_zhi_index(&self) -> i64 {
        self.year_zhi_index
    }
    pub const fn year_gan_index_by_li_chun(&self) -> i64 {
        self.year_gan_index_by_li_chun
    }
    pub const fn year_zhi_index_by_li_chun(&self) -> i64 {
        self.year_zhi_index_by_li_chun
    }
    pub const fn year_gan_index_exact(&self) -> i64 {
        self.year_gan_index_exact
    }
    pub const fn year_zhi_index_exact(&self) -> i64 {
        self.year_zhi_index_exact
    }
    pub const fn month_gan_index(&self) -> i64 {
        self.month_gan_index
    }
    pub const fn month_zhi_index(&self) -> i64 {
        self.month_zhi_index
    }
    pub const fn month_gan_index_exact(&self) -> i64 {
        self.month_gan_index_exact
    }
    pub const fn month_zhi_index_exact(&self) -> i64 {
        self.month_zhi_index_exact
    }
    pub const fn day_gan_index(&self) -> i64 {
        self.day_gan_index
    }
    pub const fn day_zhi_index(&self) -> i64 {
        self.day_zhi_index
    }
    pub const fn day_gan_index_exact(&self) -> i64 {
        self.day_gan_index_exact
    }
    pub const fn day_zhi_index_exact(&self) -> i64 {
        self.day_zhi_index_exact
    }
    pub const fn day_gan_index_exact2(&self) -> i64 {
        self.day_gan_index_exact2
    }
    pub const fn day_zhi_index_exact2(&self) -> i64 {
        self.day_zhi_index_exact2
    }
    pub const fn time_gan_index(&self) -> i64 {
        self.time_gan_index
    }
    pub const fn time_zhi_index(&self) -> i64 {
        self.time_zhi_index
    }

    // ---- 干支 ----
    pub fn year_gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.year_gan_index + 1) as usize]
    }
    pub fn year_gan_by_li_chun(&self) -> &'static str {
        lunar_util::tables::GAN[(self.year_gan_index_by_li_chun + 1) as usize]
    }
    pub fn year_gan_exact(&self) -> &'static str {
        lunar_util::tables::GAN[(self.year_gan_index_exact + 1) as usize]
    }
    pub fn year_zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.year_zhi_index + 1) as usize]
    }
    pub fn year_zhi_by_li_chun(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.year_zhi_index_by_li_chun + 1) as usize]
    }
    pub fn year_zhi_exact(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.year_zhi_index_exact + 1) as usize]
    }
    pub fn year_in_gan_zhi(&self) -> String {
        format!("{}{}", self.year_gan(), self.year_zhi())
    }
    pub fn year_in_gan_zhi_by_li_chun(&self) -> String {
        format!("{}{}", self.year_gan_by_li_chun(), self.year_zhi_by_li_chun())
    }
    pub fn year_in_gan_zhi_exact(&self) -> String {
        format!("{}{}", self.year_gan_exact(), self.year_zhi_exact())
    }

    pub fn month_gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.month_gan_index + 1) as usize]
    }
    pub fn month_gan_exact(&self) -> &'static str {
        lunar_util::tables::GAN[(self.month_gan_index_exact + 1) as usize]
    }
    pub fn month_zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.month_zhi_index + 1) as usize]
    }
    pub fn month_zhi_exact(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.month_zhi_index_exact + 1) as usize]
    }
    pub fn month_in_gan_zhi(&self) -> String {
        format!("{}{}", self.month_gan(), self.month_zhi())
    }
    pub fn month_in_gan_zhi_exact(&self) -> String {
        format!("{}{}", self.month_gan_exact(), self.month_zhi_exact())
    }

    pub fn day_gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.day_gan_index + 1) as usize]
    }
    pub fn day_gan_exact(&self) -> &'static str {
        lunar_util::tables::GAN[(self.day_gan_index_exact + 1) as usize]
    }
    pub fn day_gan_exact2(&self) -> &'static str {
        lunar_util::tables::GAN[(self.day_gan_index_exact2 + 1) as usize]
    }
    pub fn day_zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.day_zhi_index + 1) as usize]
    }
    pub fn day_zhi_exact(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.day_zhi_index_exact + 1) as usize]
    }
    pub fn day_zhi_exact2(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.day_zhi_index_exact2 + 1) as usize]
    }
    pub fn day_in_gan_zhi(&self) -> String {
        format!("{}{}", self.day_gan(), self.day_zhi())
    }
    pub fn day_in_gan_zhi_exact(&self) -> String {
        format!("{}{}", self.day_gan_exact(), self.day_zhi_exact())
    }
    pub fn day_in_gan_zhi_exact2(&self) -> String {
        format!("{}{}", self.day_gan_exact2(), self.day_zhi_exact2())
    }

    pub fn time_gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.time_gan_index + 1) as usize]
    }
    pub fn time_zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.time_zhi_index + 1) as usize]
    }
    pub fn time_in_gan_zhi(&self) -> String {
        format!("{}{}", self.time_gan(), self.time_zhi())
    }

    // ---- 纳音 ----
    pub fn year_nayin(&self) -> &'static str {
        lunar_util::nayin(&self.year_in_gan_zhi())
    }
    pub fn month_nayin(&self) -> &'static str {
        lunar_util::nayin(&self.month_in_gan_zhi())
    }
    pub fn day_nayin(&self) -> &'static str {
        lunar_util::nayin(&self.day_in_gan_zhi())
    }
    pub fn time_nayin(&self) -> &'static str {
        lunar_util::nayin(&self.time_in_gan_zhi())
    }

    // ---- 生肖 ----
    pub fn year_sheng_xiao(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.year_zhi_index + 1) as usize]
    }
    pub fn year_sheng_xiao_by_li_chun(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.year_zhi_index_by_li_chun + 1) as usize]
    }
    pub fn year_sheng_xiao_exact(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.year_zhi_index_exact + 1) as usize]
    }
    pub fn month_sheng_xiao(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.month_zhi_index + 1) as usize]
    }
    pub fn day_sheng_xiao(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.day_zhi_index + 1) as usize]
    }
    pub fn time_sheng_xiao(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.time_zhi_index + 1) as usize]
    }

    // ---- 中文表示 ----
    pub fn year_in_chinese(&self) -> String {
        self.year
            .to_string()
            .chars()
            .map(|c| {
                let d = c.to_digit(10).unwrap_or(0) as usize;
                lunar_util::tables::NUMBER[d]
            })
            .collect()
    }
    pub fn month_in_chinese(&self) -> String {
        if self.month < 0 {
            format!("闰{}", lunar_util::tables::MONTH[(-self.month) as usize])
        } else {
            lunar_util::tables::MONTH[self.month as usize].to_string()
        }
    }
    pub fn day_in_chinese(&self) -> &'static str {
        lunar_util::tables::DAY[self.day as usize]
    }
    pub fn season(&self) -> &'static str {
        let m = self.month.abs();
        lunar_util::tables::SEASON[m as usize]
    }

    // ---- 节气 ----
    pub fn jie(&self) -> &'static str {
        let mut jie = "冬至"; // 空字符串等价：用冬至占位，最终若没找到由长度判断
        let mut found = false;
        let mut i = 0;
        while i < JIE_QI_IN_USE.len() {
            let key = JIE_QI_IN_USE[i];
            let d = self.jq(key);
            if d.year() == self.solar.year() && d.month() == self.solar.month() && d.day() == self.solar.day() {
                jie = key;
                found = true;
                break;
            }
            i += 2;
        }
        if found { convert_jie_qi(jie) } else { "" }
    }
    pub fn qi(&self) -> &'static str {
        let mut qi = "冬至";
        let mut found = false;
        let mut i = 1;
        while i < JIE_QI_IN_USE.len() {
            let key = JIE_QI_IN_USE[i];
            let d = self.jq(key);
            if d.year() == self.solar.year() && d.month() == self.solar.month() && d.day() == self.solar.day() {
                qi = key;
                found = true;
                break;
            }
            i += 2;
        }
        if found { convert_jie_qi(qi) } else { "" }
    }
    pub fn jie_qi(&self) -> &'static str {
        let mut name = "冬至";
        let mut found = false;
        for key in JIE_QI_IN_USE.iter() {
            let d = self.jq(key);
            if d.year() == self.solar.year() && d.month() == self.solar.month() && d.day() == self.solar.day() {
                name = key;
                found = true;
                break;
            }
        }
        if found { convert_jie_qi(name) } else { "" }
    }
    pub fn jie_qi_table(&self) -> &HashMap<&'static str, Solar> {
        &self.jie_qi
    }
    pub fn jie_qi_list(&self) -> Vec<&'static str> {
        JIE_QI_IN_USE.to_vec()
    }
    pub fn current_jie_qi(&self) -> Option<JieQi> {
        let name = self.jie_qi();
        if !name.is_empty() { Some(JieQi::new(name, self.solar)) } else { None }
    }

    fn near_jie_qi(&self, forward: bool, conditions: Option<&[&'static str]>, whole_day: bool) -> Option<JieQi> {
        let filters: HashMap<&'static str, bool> =
            conditions.map(|c| c.iter().map(|x| (*x, true)).collect()).unwrap_or_default();
        let filter = !filters.is_empty();
        let today = if whole_day { self.solar.to_ymd() } else { self.solar.to_ymd_hms() };
        let mut near: Option<(&'static str, Solar)> = None;
        for key in JIE_QI_IN_USE.iter() {
            let jq_name = convert_jie_qi(key);
            if filter && !filters.contains_key(jq_name) {
                continue;
            }
            let solar = self.jq(key);
            let day = if whole_day { solar.to_ymd() } else { solar.to_ymd_hms() };
            if forward {
                if day <= today {
                    continue;
                }
                near = match near {
                    None => Some((jq_name, solar)),
                    Some((_, n)) => {
                        let near_day = if whole_day { n.to_ymd() } else { n.to_ymd_hms() };
                        if day < near_day { Some((jq_name, solar)) } else { Some((jq_name, n)) }
                    }
                };
            } else {
                if day > today {
                    continue;
                }
                near = match near {
                    None => Some((jq_name, solar)),
                    Some((_, n)) => {
                        let near_day = if whole_day { n.to_ymd() } else { n.to_ymd_hms() };
                        if day > near_day { Some((jq_name, solar)) } else { Some((jq_name, n)) }
                    }
                };
            }
        }
        near.map(|(name, solar)| JieQi::new(name, solar))
    }

    pub fn next_jie(&self) -> Option<JieQi> {
        self.next_jie_by_whole_day(false)
    }
    pub fn next_jie_by_whole_day(&self, whole_day: bool) -> Option<JieQi> {
        let conds: Vec<&'static str> = (0..JIE_QI_IN_USE.len() / 2).map(|i| JIE_QI_IN_USE[i * 2]).collect();
        self.near_jie_qi(true, Some(&conds), whole_day)
    }
    pub fn prev_jie(&self) -> Option<JieQi> {
        self.prev_jie_by_whole_day(false)
    }
    pub fn prev_jie_by_whole_day(&self, whole_day: bool) -> Option<JieQi> {
        let conds: Vec<&'static str> = (0..JIE_QI_IN_USE.len() / 2).map(|i| JIE_QI_IN_USE[i * 2]).collect();
        self.near_jie_qi(false, Some(&conds), whole_day)
    }
    pub fn next_qi(&self) -> Option<JieQi> {
        self.next_qi_by_whole_day(false)
    }
    pub fn next_qi_by_whole_day(&self, whole_day: bool) -> Option<JieQi> {
        let conds: Vec<&'static str> = (0..JIE_QI_IN_USE.len() / 2).map(|i| JIE_QI_IN_USE[i * 2 + 1]).collect();
        self.near_jie_qi(true, Some(&conds), whole_day)
    }
    pub fn prev_qi(&self) -> Option<JieQi> {
        self.prev_qi_by_whole_day(false)
    }
    pub fn prev_qi_by_whole_day(&self, whole_day: bool) -> Option<JieQi> {
        let conds: Vec<&'static str> = (0..JIE_QI_IN_USE.len() / 2).map(|i| JIE_QI_IN_USE[i * 2 + 1]).collect();
        self.near_jie_qi(false, Some(&conds), whole_day)
    }
    pub fn next_jie_qi(&self) -> Option<JieQi> {
        self.next_jie_qi_by_whole_day(false)
    }
    pub fn next_jie_qi_by_whole_day(&self, whole_day: bool) -> Option<JieQi> {
        self.near_jie_qi(true, None, whole_day)
    }
    pub fn prev_jie_qi(&self) -> Option<JieQi> {
        self.prev_jie_qi_by_whole_day(false)
    }
    pub fn prev_jie_qi_by_whole_day(&self, whole_day: bool) -> Option<JieQi> {
        self.near_jie_qi(false, None, whole_day)
    }

    // ---- 节日 ----
    pub fn festivals(&self) -> Vec<&'static str> {
        let mut l = Vec::new();
        let key = format!("{}-{}", self.month, self.day);
        if let Some(f) = lunar_util::maps::FESTIVAL.get(key.as_str()) {
            l.push(*f);
        }
        let m = self.month.abs();
        if m == 12 && self.day >= 29 && self.year != self.next(1).year {
            l.push("除夕");
        }
        l
    }
    pub fn other_festivals(&self) -> Vec<&'static str> {
        let mut l = Vec::new();
        let key = format!("{}-{}", self.month, self.day);
        if let Some(f) = lunar_util::maps::OTHER_FESTIVAL.get(key.as_str()) {
            l.extend(f.iter().copied());
        }
        let solar_ymd = self.solar.to_ymd();
        let qing_ming = self.jq("清明");
        if solar_ymd == qing_ming.next_day(-1).to_ymd() {
            l.push("寒食节");
        }
        let li_chun = self.jq("立春");
        let mut offset = 4_i64 - li_chun.lunar().day_gan_index();
        if offset < 0 {
            offset += 10;
        }
        if solar_ymd == li_chun.next_day((offset + 40) as i32).to_ymd() {
            l.push("春社");
        }
        let li_qiu = self.jq("立秋");
        let mut offset = 4_i64 - li_qiu.lunar().day_gan_index();
        if offset < 0 {
            offset += 10;
        }
        if solar_ymd == li_qiu.next_day((offset + 40) as i32).to_ymd() {
            l.push("秋社");
        }
        l
    }

    // ---- 彭祖 ----
    pub fn peng_zu_gan(&self) -> &'static str {
        lunar_util::tables::PENGZU_GAN[(self.day_gan_index + 1) as usize]
    }
    pub fn peng_zu_zhi(&self) -> &'static str {
        lunar_util::tables::PENGZU_ZHI[(self.day_zhi_index + 1) as usize]
    }

    // ---- 方位（日）----
    pub fn day_position_xi(&self) -> &'static str {
        lunar_util::tables::POSITION_XI[(self.day_gan_index + 1) as usize]
    }
    pub fn day_position_xi_desc(&self) -> &'static str {
        lunar_util::position_desc(self.day_position_xi())
    }
    pub fn day_position_yang_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YANG_GUI[(self.day_gan_index + 1) as usize]
    }
    pub fn day_position_yang_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.day_position_yang_gui())
    }
    pub fn day_position_yin_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YIN_GUI[(self.day_gan_index + 1) as usize]
    }
    pub fn day_position_yin_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.day_position_yin_gui())
    }
    pub fn day_position_fu(&self) -> &'static str {
        self.day_position_fu_by_sect(2)
    }
    pub fn day_position_fu_by_sect(&self, sect: u8) -> &'static str {
        let offset = (self.day_gan_index + 1) as usize;
        if sect == 1 { lunar_util::tables::POSITION_FU[offset] } else { lunar_util::tables::POSITION_FU_2[offset] }
    }
    pub fn day_position_fu_desc(&self) -> &'static str {
        lunar_util::position_desc(self.day_position_fu_by_sect(2))
    }
    pub fn day_position_cai(&self) -> &'static str {
        lunar_util::tables::POSITION_CAI[(self.day_gan_index + 1) as usize]
    }
    pub fn day_position_cai_desc(&self) -> &'static str {
        lunar_util::position_desc(self.day_position_cai())
    }

    pub fn year_position_tai_sui(&self) -> &'static str {
        self.year_position_tai_sui_by_sect(2)
    }
    pub fn year_position_tai_sui_by_sect(&self, sect: u8) -> &'static str {
        let year_zhi_index = match sect {
            1 => self.year_zhi_index,
            3 => self.year_zhi_index_exact,
            _ => self.year_zhi_index_by_li_chun,
        };
        lunar_util::tables::POSITION_TAI_SUI_YEAR[year_zhi_index as usize]
    }
    pub fn year_position_tai_sui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.year_position_tai_sui_by_sect(2))
    }

    fn month_position_tai_sui_inner(&self, month_zhi_index: i64, month_gan_index: i64) -> &'static str {
        let mut m = month_zhi_index - lunar_util::BASE_MONTH_ZHI_INDEX;
        if m < 0 {
            m += 12;
        }
        m %= 4;
        match m {
            0 => "艮",
            2 => "坤",
            3 => "巽",
            _ => lunar_util::tables::POSITION_GAN[month_gan_index as usize],
        }
    }
    pub fn month_position_tai_sui(&self) -> &'static str {
        self.month_position_tai_sui_by_sect(2)
    }
    pub fn month_position_tai_sui_by_sect(&self, sect: u8) -> &'static str {
        let (mzi, mgi) = match sect {
            3 => (self.month_zhi_index_exact, self.month_gan_index_exact),
            _ => (self.month_zhi_index, self.month_gan_index),
        };
        self.month_position_tai_sui_inner(mzi, mgi)
    }
    pub fn month_position_tai_sui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.month_position_tai_sui())
    }

    fn day_position_tai_sui_inner(&self, day_in_gan_zhi: &str, year_zhi_index: i64) -> &'static str {
        if "甲子，乙丑，丙寅，丁卯，戊辰，己巳".contains(day_in_gan_zhi) {
            "震"
        } else if "丙子，丁丑，戊寅，己卯，庚辰，辛巳".contains(day_in_gan_zhi) {
            "离"
        } else if "戊子，己丑，庚寅，辛卯，壬辰，癸巳".contains(day_in_gan_zhi) {
            "中"
        } else if "庚子，辛丑，壬寅，癸卯，甲辰，乙巳".contains(day_in_gan_zhi) {
            "兑"
        } else if "壬子，癸丑，甲寅，乙卯，丙辰，丁巳".contains(day_in_gan_zhi) {
            "坎"
        } else {
            lunar_util::tables::POSITION_TAI_SUI_YEAR[year_zhi_index as usize]
        }
    }
    pub fn day_position_tai_sui(&self) -> &'static str {
        self.day_position_tai_sui_by_sect(2)
    }
    pub fn day_position_tai_sui_by_sect(&self, sect: u8) -> &'static str {
        let (day_in_gan_zhi, year_zhi_index) = match sect {
            1 => (self.day_in_gan_zhi(), self.year_zhi_index),
            3 => (self.day_in_gan_zhi(), self.year_zhi_index_exact),
            _ => (self.day_in_gan_zhi_exact2(), self.year_zhi_index_by_li_chun),
        };
        self.day_position_tai_sui_inner(&day_in_gan_zhi, year_zhi_index)
    }
    pub fn day_position_tai_sui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.day_position_tai_sui())
    }

    // ---- 方位（时）----
    pub fn time_position_xi(&self) -> &'static str {
        lunar_util::tables::POSITION_XI[(self.time_gan_index + 1) as usize]
    }
    pub fn time_position_xi_desc(&self) -> &'static str {
        lunar_util::position_desc(self.time_position_xi())
    }
    pub fn time_position_yang_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YANG_GUI[(self.time_gan_index + 1) as usize]
    }
    pub fn time_position_yang_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.time_position_yang_gui())
    }
    pub fn time_position_yin_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YIN_GUI[(self.time_gan_index + 1) as usize]
    }
    pub fn time_position_yin_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.time_position_yin_gui())
    }
    pub fn time_position_fu(&self) -> &'static str {
        lunar_util::tables::POSITION_FU[(self.time_gan_index + 1) as usize]
    }
    pub fn time_position_fu_desc(&self) -> &'static str {
        lunar_util::position_desc(self.time_position_fu())
    }
    pub fn time_position_cai(&self) -> &'static str {
        lunar_util::tables::POSITION_CAI[(self.time_gan_index + 1) as usize]
    }
    pub fn time_position_cai_desc(&self) -> &'static str {
        lunar_util::position_desc(self.time_position_cai())
    }

    // ---- 胎神 ----
    pub fn day_position_tai(&self) -> &'static str {
        let idx = lunar_util::get_jia_zi_index(&self.day_in_gan_zhi());
        lunar_util::tables::POSITION_TAI_DAY[idx as usize]
    }
    pub fn month_position_tai(&self) -> &'static str {
        if self.month < 0 {
            return "";
        }
        lunar_util::tables::POSITION_TAI_MONTH[(self.month - 1) as usize]
    }

    // ---- 冲煞 ----
    pub fn day_chong(&self) -> &'static str {
        lunar_util::tables::CHONG[self.day_zhi_index as usize]
    }
    pub fn day_chong_gan(&self) -> &'static str {
        lunar_util::tables::CHONG_GAN[self.day_gan_index as usize]
    }
    pub fn day_chong_gan_tie(&self) -> &'static str {
        lunar_util::tables::CHONG_GAN_TIE[self.day_gan_index as usize]
    }
    pub fn day_chong_sheng_xiao(&self) -> &'static str {
        let chong = self.day_chong();
        for (i, v) in lunar_util::tables::ZHI.iter().enumerate() {
            if *v == chong {
                return lunar_util::tables::SHENG_XIAO[i];
            }
        }
        ""
    }
    pub fn day_chong_desc(&self) -> String {
        format!("({}{}){}", self.day_chong_gan(), self.day_chong(), self.day_chong_sheng_xiao())
    }
    pub fn day_sha(&self) -> &'static str {
        lunar_util::sha(self.day_zhi())
    }
    pub fn time_chong(&self) -> &'static str {
        lunar_util::tables::CHONG[self.time_zhi_index as usize]
    }
    pub fn time_chong_gan(&self) -> &'static str {
        lunar_util::tables::CHONG_GAN[self.time_gan_index as usize]
    }
    pub fn time_chong_gan_tie(&self) -> &'static str {
        lunar_util::tables::CHONG_GAN_TIE[self.time_gan_index as usize]
    }
    pub fn time_chong_sheng_xiao(&self) -> &'static str {
        let chong = self.time_chong();
        for (i, v) in lunar_util::tables::ZHI.iter().enumerate() {
            if *v == chong {
                return lunar_util::tables::SHENG_XIAO[i];
            }
        }
        ""
    }
    pub fn time_chong_desc(&self) -> String {
        format!("({}{}){}", self.time_chong_gan(), self.time_chong(), self.time_chong_sheng_xiao())
    }
    pub fn time_sha(&self) -> &'static str {
        lunar_util::sha(self.time_zhi())
    }

    // ---- 天神 ----
    pub fn day_tian_shen(&self) -> &'static str {
        let off =
            lunar_util::ZHI_TIAN_SHEN_OFFSET[lunar_util::find(self.month_zhi(), lunar_util::tables::ZHI, 0) as usize];
        lunar_util::tables::TIAN_SHEN[((self.day_zhi_index + off) % 12 + 1) as usize]
    }
    pub fn time_tian_shen(&self) -> &'static str {
        let off = lunar_util::ZHI_TIAN_SHEN_OFFSET
            [lunar_util::find(self.day_zhi_exact(), lunar_util::tables::ZHI, 0) as usize];
        lunar_util::tables::TIAN_SHEN[((self.time_zhi_index + off) % 12 + 1) as usize]
    }
    pub fn day_tian_shen_type(&self) -> &'static str {
        lunar_util::tian_shen_type(self.day_tian_shen())
    }
    pub fn time_tian_shen_type(&self) -> &'static str {
        lunar_util::tian_shen_type(self.time_tian_shen())
    }
    pub fn day_tian_shen_luck(&self) -> &'static str {
        lunar_util::tian_shen_type_luck(self.day_tian_shen_type())
    }
    pub fn time_tian_shen_luck(&self) -> &'static str {
        lunar_util::tian_shen_type_luck(self.time_tian_shen_type())
    }

    // ---- 建除 / 二十八宿 ----
    pub fn zhi_xing(&self) -> &'static str {
        let mut offset = self.day_zhi_index - self.month_zhi_index;
        if offset < 0 {
            offset += 12;
        }
        lunar_util::tables::ZHI_XING[(offset + 1) as usize]
    }
    pub fn xiu(&self) -> &'static str {
        // 查找键为 `{dayZhi}{week}`（如 "戌3"）；表键可能被 CJK 排版工具写作 "戌 3"，故两种都试。
        let k0 = format!("{}{}", self.day_zhi(), self.week_index);
        let v = lunar_util::xiu(&k0);
        if !v.is_empty() {
            return v;
        }
        let k1 = format!("{} {}", self.day_zhi(), self.week_index);
        lunar_util::xiu(&k1)
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

    // ---- 宜忌 / 吉神凶煞 ----
    pub fn day_yi(&self) -> Vec<&'static str> {
        self.day_yi_by_sect(1)
    }
    pub fn day_yi_by_sect(&self, sect: u8) -> Vec<&'static str> {
        let month_gan_zhi = if sect == 2 { self.month_in_gan_zhi_exact() } else { self.month_in_gan_zhi() };
        lunar_util::get_day_yi(&month_gan_zhi, &self.day_in_gan_zhi())
    }
    pub fn day_ji(&self) -> Vec<&'static str> {
        self.day_ji_by_sect(1)
    }
    pub fn day_ji_by_sect(&self, sect: u8) -> Vec<&'static str> {
        let month_gan_zhi = if sect == 2 { self.month_in_gan_zhi_exact() } else { self.month_in_gan_zhi() };
        lunar_util::get_day_ji(&month_gan_zhi, &self.day_in_gan_zhi())
    }
    pub fn day_ji_shen(&self) -> Vec<&'static str> {
        lunar_util::get_day_ji_shen(self.month_zhi_index, &self.day_in_gan_zhi())
    }
    pub fn day_xiong_sha(&self) -> Vec<&'static str> {
        lunar_util::get_day_xiong_sha(self.month_zhi_index, &self.day_in_gan_zhi())
    }
    pub fn time_yi(&self) -> Vec<&'static str> {
        lunar_util::get_time_yi(&self.day_in_gan_zhi_exact(), &self.time_in_gan_zhi())
    }
    pub fn time_ji(&self) -> Vec<&'static str> {
        lunar_util::get_time_ji(&self.day_in_gan_zhi_exact(), &self.time_in_gan_zhi())
    }

    // ---- 月相 / 六曜 / 星期 ----
    pub fn yue_xiang(&self) -> &'static str {
        lunar_util::tables::YUE_XIANG[self.day as usize]
    }
    pub fn liu_yao(&self) -> &'static str {
        let month = self.month.abs();
        let idx = ((month + self.day - 2) % 6) as usize;
        lunar_util::tables::LIU_YAO[idx]
    }
    pub fn week(&self) -> i32 {
        self.week_index
    }
    pub fn week_in_chinese(&self) -> &'static str {
        solar_util::WEEK[self.week_index as usize]
    }

    // ---- 旬 / 空亡 ----
    pub fn year_xun(&self) -> &'static str {
        lunar_util::get_xun(&self.year_in_gan_zhi())
    }
    pub fn year_xun_by_li_chun(&self) -> &'static str {
        lunar_util::get_xun(&self.year_in_gan_zhi_by_li_chun())
    }
    pub fn year_xun_exact(&self) -> &'static str {
        lunar_util::get_xun(&self.year_in_gan_zhi_exact())
    }
    pub fn year_xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.year_in_gan_zhi())
    }
    pub fn year_xun_kong_by_li_chun(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.year_in_gan_zhi_by_li_chun())
    }
    pub fn year_xun_kong_exact(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.year_in_gan_zhi_exact())
    }
    pub fn month_xun(&self) -> &'static str {
        lunar_util::get_xun(&self.month_in_gan_zhi())
    }
    pub fn month_xun_exact(&self) -> &'static str {
        lunar_util::get_xun(&self.month_in_gan_zhi_exact())
    }
    pub fn month_xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.month_in_gan_zhi())
    }
    pub fn month_xun_kong_exact(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.month_in_gan_zhi_exact())
    }
    pub fn day_xun(&self) -> &'static str {
        lunar_util::get_xun(&self.day_in_gan_zhi())
    }
    pub fn day_xun_exact(&self) -> &'static str {
        lunar_util::get_xun(&self.day_in_gan_zhi_exact())
    }
    pub fn day_xun_exact2(&self) -> &'static str {
        lunar_util::get_xun(&self.day_in_gan_zhi_exact2())
    }
    pub fn day_xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.day_in_gan_zhi())
    }
    pub fn day_xun_kong_exact(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.day_in_gan_zhi_exact())
    }
    pub fn day_xun_kong_exact2(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.day_in_gan_zhi_exact2())
    }
    pub fn time_xun(&self) -> &'static str {
        lunar_util::get_xun(&self.time_in_gan_zhi())
    }
    pub fn time_xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.time_in_gan_zhi())
    }

    // ---- 日禄 ----
    pub fn day_lu(&self) -> String {
        let gan = lunar_util::lu(self.day_gan());
        let mut lu = format!("{gan}命互禄");
        let zhi = lunar_util::lu(self.day_zhi());
        if !zhi.is_empty() {
            lu.push(' ');
            lu.push_str(zhi);
            lu.push_str("命进禄");
        }
        lu
    }

    // ---- 数九 / 三伏 / 候 ----
    pub fn shu_jiu(&self) -> Option<ShuJiu> {
        let current = Solar::from_ymd(self.solar.year(), self.solar.month(), self.solar.day()).unwrap_or(self.solar);
        let dong_zhi = self.jq("DONG_ZHI");
        let mut start = Solar::from_ymd(dong_zhi.year(), dong_zhi.month(), dong_zhi.day()).unwrap_or(dong_zhi);
        if current.is_before(&start) {
            let dz = self.jq("冬至");
            start = Solar::from_ymd(dz.year(), dz.month(), dz.day()).unwrap_or(dz);
        }
        let end = start.next_day(81);
        if current.is_before(&start) || !current.is_before(&end) {
            return None;
        }
        let days = current.subtract(&start);
        Some(ShuJiu::new(format!("{}九", lunar_util::tables::NUMBER[(days / 9 + 1) as usize]), days % 9 + 1))
    }

    pub fn fu(&self) -> Option<Fu> {
        let current = Solar::from_ymd(self.solar.year(), self.solar.month(), self.solar.day()).unwrap_or(self.solar);
        let xia_zhi = self.jq("夏至");
        let li_qiu = self.jq("立秋");
        let mut start = Solar::from_ymd(xia_zhi.year(), xia_zhi.month(), xia_zhi.day()).unwrap_or(xia_zhi);
        let mut add = 6_i64 - xia_zhi.lunar().day_gan_index();
        if add < 0 {
            add += 10;
        }
        add += 20;
        start = start.next_day(add as i32);
        if current.is_before(&start) {
            return None;
        }
        let mut days = current.subtract(&start);
        if days < 10 {
            return Some(Fu::new("初伏", days + 1));
        }
        start = start.next_day(10);
        days = current.subtract(&start);
        if days < 10 {
            return Some(Fu::new("中伏", days + 1));
        }
        start = start.next_day(10);
        days = current.subtract(&start);
        let li_qiu_solar = Solar::from_ymd(li_qiu.year(), li_qiu.month(), li_qiu.day()).unwrap_or(li_qiu);
        if li_qiu_solar.is_after(&start) {
            if days < 10 {
                return Some(Fu::new("中伏", days + 11));
            }
            start = start.next_day(10);
            days = current.subtract(&start);
        }
        if days < 10 {
            return Some(Fu::new("末伏", days + 1));
        }
        None
    }

    pub fn hou(&self) -> String {
        let jq = self.prev_jie_qi_by_whole_day(true).unwrap();
        let max = lunar_util::tables::HOU.len() as i32 - 1;
        let mut offset = self.solar.subtract(&jq.solar()) / 5;
        if offset > max {
            offset = max;
        }
        format!("{} {}", jq.name(), lunar_util::tables::HOU[offset as usize])
    }
    pub fn wu_hou(&self) -> &'static str {
        let jq = self.prev_jie_qi_by_whole_day(true).unwrap();
        let mut offset = 0_i64;
        for (i, v) in JIE_QI.iter().enumerate() {
            if *v == jq.name() {
                offset = i as i64;
                break;
            }
        }
        let mut index = (self.solar.subtract(&jq.solar()) / 5) as i64;
        if index > 2 {
            index = 2;
        }
        let wu_hou_len = lunar_util::tables::WU_HOU.len() as i64;
        lunar_util::tables::WU_HOU[((offset * 3 + index) % wu_hou_len) as usize]
    }

    // ---- 九星 ----
    fn year_nine_star_inner(&self, year_in_gan_zhi: &str) -> NineStar {
        let index_exact = lunar_util::get_jia_zi_index(year_in_gan_zhi) + 1;
        let index = lunar_util::get_jia_zi_index(&self.year_in_gan_zhi()) + 1;
        let mut year_offset = index_exact - index;
        if year_offset > 1 {
            year_offset -= 60;
        } else if year_offset < -1 {
            year_offset += 60;
        }
        let yuan = ((self.year + year_offset as i32 + 2696) / 60) % 3;
        let mut offset = (62 + yuan as i64 * 3 - index_exact) % 9;
        if offset == 0 {
            offset = 9;
        }
        NineStar::from_index(offset - 1)
    }
    pub fn year_nine_star(&self) -> NineStar {
        self.year_nine_star_by_sect(2)
    }
    pub fn year_nine_star_by_sect(&self, sect: u8) -> NineStar {
        let g = match sect {
            1 => self.year_in_gan_zhi(),
            3 => self.year_in_gan_zhi_exact(),
            _ => self.year_in_gan_zhi_by_li_chun(),
        };
        self.year_nine_star_inner(&g)
    }
    fn month_nine_star_inner(&self, year_zhi_index: i64, month_zhi_index: i64) -> NineStar {
        let index = year_zhi_index % 3;
        let mut n = 27 - index * 3;
        if month_zhi_index < lunar_util::BASE_MONTH_ZHI_INDEX {
            n -= 3;
        }
        let offset = (n - month_zhi_index) % 9;
        NineStar::from_index(offset)
    }
    pub fn month_nine_star(&self) -> NineStar {
        self.month_nine_star_by_sect(2)
    }
    pub fn month_nine_star_by_sect(&self, sect: u8) -> NineStar {
        let (yzi, mzi) = match sect {
            1 => (self.year_zhi_index, self.month_zhi_index),
            3 => (self.year_zhi_index_exact, self.month_zhi_index_exact),
            _ => (self.year_zhi_index_by_li_chun, self.month_zhi_index),
        };
        self.month_nine_star_inner(yzi, mzi)
    }
    pub fn day_nine_star(&self) -> NineStar {
        let solar_ymd = self.solar.to_ymd();
        let dong_zhi = self.jq("冬至");
        let dong_zhi2 = self.jq("DONG_ZHI");
        let xia_zhi = self.jq("夏至");
        let dong_zhi_index = lunar_util::get_jia_zi_index(&dong_zhi.lunar().day_in_gan_zhi());
        let dong_zhi_index2 = lunar_util::get_jia_zi_index(&dong_zhi2.lunar().day_in_gan_zhi());
        let xia_zhi_index = lunar_util::get_jia_zi_index(&xia_zhi.lunar().day_in_gan_zhi());
        let solar_shun_bai = if dong_zhi_index > 29 {
            dong_zhi.next_day((60 - dong_zhi_index) as i32)
        } else {
            dong_zhi.next_day((-dong_zhi_index) as i32)
        };
        let solar_shun_bai_ymd = solar_shun_bai.to_ymd();
        let solar_shun_bai2 = if dong_zhi_index2 > 29 {
            dong_zhi2.next_day((60 - dong_zhi_index2) as i32)
        } else {
            dong_zhi2.next_day((-dong_zhi_index2) as i32)
        };
        let solar_shun_bai_ymd2 = solar_shun_bai2.to_ymd();
        let solar_ni_zi = if xia_zhi_index > 29 {
            xia_zhi.next_day((60 - xia_zhi_index) as i32)
        } else {
            xia_zhi.next_day((-xia_zhi_index) as i32)
        };
        let solar_ni_zi_ymd = solar_ni_zi.to_ymd();
        let offset = if solar_ymd >= solar_shun_bai_ymd && solar_ymd < solar_ni_zi_ymd {
            self.solar.subtract(&solar_shun_bai) % 9
        } else if solar_ymd >= solar_ni_zi_ymd && solar_ymd < solar_shun_bai_ymd2 {
            8 - (self.solar.subtract(&solar_ni_zi) % 9)
        } else if solar_ymd >= solar_shun_bai_ymd2 {
            self.solar.subtract(&solar_shun_bai2) % 9
        } else if solar_ymd < solar_shun_bai_ymd {
            (8 + solar_shun_bai.subtract(&self.solar)) % 9
        } else {
            0
        };
        NineStar::from_index(offset as i64)
    }
    pub fn time_nine_star(&self) -> NineStar {
        let solar_ymd = self.solar.to_ymd();
        let mut asc = false;
        if solar_ymd >= self.jq("冬至").to_ymd() && solar_ymd < self.jq("夏至").to_ymd() {
            asc = true;
        } else if solar_ymd >= self.jq("DONG_ZHI").to_ymd() {
            asc = true;
        }
        let mut start: i64 = if asc { 6 } else { 2 };
        let day_zhi = self.day_zhi();
        if "子午卯酉".contains(day_zhi) {
            start = if asc { 0 } else { 8 };
        } else if "辰戌丑未".contains(day_zhi) {
            start = if asc { 3 } else { 5 };
        }
        let index = if asc { start + self.time_zhi_index } else { start + 9 - self.time_zhi_index };
        NineStar::from_index(index % 9)
    }

    // ---- 包装类型 ----
    pub fn eight_char(&self) -> EightChar<'_> {
        EightChar::from_lunar(self)
    }

    /// 佛历。
    pub fn foto(&self) -> crate::foto::Foto<'_> {
        crate::foto::Foto::from_lunar(self)
    }

    /// 道历。
    pub fn tao(&self) -> crate::tao::Tao<'_> {
        crate::tao::Tao::from_lunar(self)
    }

    /// 当前时辰。
    pub fn time(&self) -> LunarTime<'_> {
        LunarTime::from_lunar(self)
    }
}

impl fmt::Display for Lunar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}年{}月{}", self.year_in_chinese(), self.month_in_chinese(), self.day_in_chinese())
    }
}

impl Lunar {
    /// 完整字符串（与 lunar-go `ToFullString` 对齐）。
    pub fn to_full_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.to_string());
        s.push(' ');
        s.push_str(&self.year_in_gan_zhi());
        s.push('(');
        s.push_str(self.year_sheng_xiao());
        s.push_str(") 年 ");
        s.push_str(&self.month_in_gan_zhi());
        s.push('(');
        s.push_str(self.month_sheng_xiao());
        s.push_str(") 月 ");
        s.push_str(&self.day_in_gan_zhi());
        s.push('(');
        s.push_str(self.day_sheng_xiao());
        s.push_str(") 日 ");
        s.push_str(self.time_zhi());
        s.push('(');
        s.push_str(self.time_sheng_xiao());
        s.push_str(") 时 纳音 [");
        s.push_str(self.year_nayin());
        s.push(' ');
        s.push_str(self.month_nayin());
        s.push(' ');
        s.push_str(self.day_nayin());
        s.push(' ');
        s.push_str(self.time_nayin());
        s.push_str("] 星期");
        s.push_str(self.week_in_chinese());
        for f in self.festivals() {
            s.push_str(" (");
            s.push_str(f);
            s.push(')');
        }
        for f in self.other_festivals() {
            s.push_str(" (");
            s.push_str(f);
            s.push(')');
        }
        let jq = self.jie_qi();
        if !jq.is_empty() {
            s.push_str(" [");
            s.push_str(jq);
            s.push(']');
        }
        s.push(' ');
        s.push_str(self.gong());
        s.push('方');
        s.push_str(self.shou());
        s.push_str(" 星宿 [");
        s.push_str(self.xiu());
        s.push_str(self.zheng());
        s.push_str(self.animal());
        s.push_str("](");
        s.push_str(self.xiu_luck());
        s.push_str(") 彭祖百忌 [");
        s.push_str(self.peng_zu_gan());
        s.push(' ');
        s.push_str(self.peng_zu_zhi());
        s.push_str("] 喜神方位 [");
        s.push_str(self.day_position_xi());
        s.push_str("](");
        s.push_str(self.day_position_xi_desc());
        s.push_str(") 阳贵神方位 [");
        s.push_str(self.day_position_yang_gui());
        s.push_str("](");
        s.push_str(self.day_position_yang_gui_desc());
        s.push_str(") 阴贵神方位 [");
        s.push_str(self.day_position_yin_gui());
        s.push_str("](");
        s.push_str(self.day_position_yin_gui_desc());
        s.push_str(") 福神方位 [");
        s.push_str(self.day_position_fu());
        s.push_str("](");
        s.push_str(self.day_position_fu_desc());
        s.push_str(") 财神方位 [");
        s.push_str(self.day_position_cai());
        s.push_str("](");
        s.push_str(self.day_position_cai_desc());
        s.push_str(") 冲 [");
        s.push_str(&self.day_chong_desc());
        s.push_str("] 煞 [");
        s.push_str(self.day_sha());
        s.push(']');
        s
    }
}
