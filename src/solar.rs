//! 阳历日期 / 时间。对应 lunar-go `calendar/Solar.go`。

use std::fmt;

use crate::holiday_util;
use crate::lunar::Lunar;
use crate::solar_util;
use crate::LunarError;

/// J2000.0 历元儒略日。
pub const J2000: f64 = 2_451_545.0;

/// 阳历日期时间。
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
        year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32,
    ) -> Result<Self, LunarError> {
        if !(1..=12).contains(&month) {
            return Err(LunarError::InvalidSolar { year, month, day, hour, minute, second });
        }
        if day < 1 || day > 31 {
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

        Self::from_ymd_hms(
            year as i32, month as i32, day as i32, hour as i32, minute as i32, second as i32,
        )
        .unwrap_or_else(|_| Self { year: year as i32, month: month as i32, day: day as i32, hour: hour as i32, minute: minute as i32, second: second as i32 })
    }

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
    pub fn is_leap_year(&self) -> bool {
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

    /// 儒略日。
    #[inline]
    pub fn julian_day(&self) -> f64 {
        solar_util::julian_day(self.year, self.month, self.day, self.hour, self.minute, self.second)
    }

    /// 转农历。
    pub fn lunar(&self) -> Lunar {
        Lunar::from_solar(*self)
    }

    /// `YYYY-MM-DD`。
    pub fn to_ymd(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// `YYYY-MM-DD HH:MM:SS`。
    pub fn to_ymd_hms(&self) -> String {
        format!(
            "{} {:02}:{:02}:{:02}",
            self.to_ymd(),
            self.hour,
            self.minute,
            self.second
        )
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

    /// 节日（几月几日 + 第 N 个星期几）。
    pub fn festivals(&self) -> Vec<&'static str> {
        let mut l = Vec::new();
        let key = format!("{}-{}", self.month, self.day);
        if let Some(f) = solar_util::FESTIVAL.get(key.as_str()) {
            l.push(*f);
        }
        let weeks = ((self.day as f64) / 7.0).ceil() as i32;
        let week = self.week();
        let key2 = format!("{}-{}-{}", self.month, weeks, week);
        if let Some(f) = solar_util::WEEK_FESTIVAL.get(key2.as_str()) {
            l.push(*f);
        }
        if self.day + 7 > solar_util::days_of_month(self.year, self.month) {
            let key3 = format!("{}-0-{}", self.month, week);
            if let Some(f) = solar_util::WEEK_FESTIVAL.get(key3.as_str()) {
                l.push(*f);
            }
        }
        l
    }

    /// 其它节日。
    pub fn other_festivals(&self) -> Vec<&'static str> {
        let key = format!("{}-{}", self.month, self.day);
        solar_util::OTHER_FESTIVAL.get(key.as_str()).cloned().unwrap_or_default()
    }

    /// 与另一日期的天数差（self - other）。
    pub fn subtract(&self, other: &Solar) -> i32 {
        solar_util::days_between(
            other.year, other.month, other.day, self.year, self.month, self.day,
        )
    }

    /// 与另一时刻的分钟差（self - other）。
    pub fn subtract_minute(&self, other: &Solar) -> i32 {
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

    pub fn is_after(&self, other: &Solar) -> bool {
        (self.year, self.month, self.day, self.hour, self.minute, self.second)
            > (other.year, other.month, other.day, other.hour, other.minute, other.second)
    }

    pub fn is_before(&self, other: &Solar) -> bool {
        solar_util::is_before(
            self.year, self.month, self.day, self.hour, self.minute, self.second, other.year,
            other.month, other.day, other.hour, other.minute, other.second,
        )
    }

    /// 推进 / 回退若干年。
    pub fn next_year(&self, years: i32) -> Solar {
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
        Solar::from_ymd_hms(y, m, d, self.hour, self.minute, self.second)
            .unwrap_or(*self)
    }

    /// 推进 / 回退若干月。
    pub fn next_month(&self, months: i32) -> Solar {
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
        Solar::from_ymd_hms(y, m, d, self.hour, self.minute, self.second).unwrap_or(*self)
    }

    /// 推进 / 回退若干天（正确处理 1582 历法改革跳过的 10 天）。
    pub fn next_day(&self, days: i32) -> Solar {
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
        Solar::from_ymd_hms(y, m, d, self.hour, self.minute, self.second).unwrap_or(*self)
    }

    /// 推进若干天，可选仅工作日。
    pub fn next(&self, days: i32, only_workday: bool) -> Solar {
        if !only_workday {
            return self.next_day(days);
        }
        let mut o = *self;
        if days != 0 {
            let mut rest = days;
            let mut add = 1;
            if days < 0 {
                rest = -days;
                add = -1;
            }
            while rest > 0 {
                o = o.next_day(add);
                let work = if let Some(holiday) = holiday_util::get_holiday_by_ymd(
                    o.year, o.month, o.day,
                ) {
                    holiday.is_work()
                } else {
                    let w = o.week();
                    !(w == 0 || w == 6)
                };
                if work {
                    rest -= 1;
                }
            }
        }
        o
    }

    /// 推进 / 回退若干小时。
    pub fn next_hour(&self, hours: i32) -> Solar {
        let h = self.hour + hours;
        let n;
        let mut hour;
        if h < 0 {
            n = -1;
            hour = -h;
        } else {
            n = 1;
            hour = h;
        }
        let mut days = hour / 24 * n;
        hour = hour % 24 * n;
        if hour < 0 {
            hour += 24;
            days -= 1;
        }
        let o = self.next_day(days);
        Solar::from_ymd_hms(o.year, o.month, o.day, hour, o.minute, o.second).unwrap_or(o)
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

/// 计算 (year, month) + months 后的 (year, month)。
pub(crate) fn next_ym(year: i32, month: i32, months: i32) -> (i32, i32) {
    let total = (year * 12 + (month - 1)) + months;
    let y = total.div_euclid(12);
    let m = total.rem_euclid(12) + 1;
    (y, m)
}
