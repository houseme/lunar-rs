//! 阳历月。对应 lunar-go `calendar/SolarMonth.go`。

use std::fmt;

use crate::solar::Solar;
use crate::solar_util;
use crate::solar_week::SolarWeek;

/// 阳历月。
#[derive(Clone, Copy, Debug)]
pub struct SolarMonth { year: i32, month: i32 }

impl SolarMonth {
    pub fn from_ym(year: i32, month: i32) -> Self { Self { year, month } }
    pub const fn year(&self) -> i32 { self.year }
    pub const fn month(&self) -> i32 { self.month }

    /// 当月每日。
    pub fn days(&self) -> Vec<Solar> {
        let first = Solar::from_ymd(self.year, self.month, 1).unwrap();
        let total = solar_util::days_of_month(self.year, self.month);
        (0..total).map(|i| first.next_day(i)).collect()
    }

    /// 当月的周列表。
    pub fn weeks(&self, start: i32) -> Vec<SolarWeek> {
        let mut out = Vec::new();
        let mut week = SolarWeek::from_ymd(self.year, self.month, 1, start);
        loop {
            out.push(week);
            week = week.next(1, false);
            let first = week.first_day();
            if first.year() > self.year || first.month() > self.month { break; }
        }
        out
    }

    /// 推进 / 回退若干月。
    pub fn next(&self, months: i32) -> Self {
        let (n, m_abs) = if months < 0 { (-1, -months) } else { (1, months) };
        let mut y = self.year + (m_abs / 12) * n;
        let mut m = self.month + (m_abs % 12) * n;
        if m > 12 { m -= 12; y += 1; } else if m < 1 { m += 12; y -= 1; }
        Self::from_ym(y, m)
    }
}

impl fmt::Display for SolarMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}-{}", self.year, self.month) }
}
