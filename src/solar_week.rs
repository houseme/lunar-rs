//! 阳历周。对应 lunar-go `calendar/SolarWeek.go`。

use std::fmt;

use crate::solar::Solar;
use crate::solar_util;

/// 阳历周（`start` 为每周起始星期，0=周日）。
#[derive(Clone, Copy, Debug)]
pub struct SolarWeek {
    year: i32,
    month: i32,
    day: i32,
    start: i32,
}

impl SolarWeek {
    pub fn from_ymd(year: i32, month: i32, day: i32, start: i32) -> Self {
        Self { year, month, day, start }
    }
    pub const fn year(&self) -> i32 { self.year }
    pub const fn month(&self) -> i32 { self.month }
    pub const fn day(&self) -> i32 { self.day }

    /// 当月第几周。
    pub fn index(&self) -> i32 {
        let mut offset = Solar::from_ymd(self.year, self.month, 1).unwrap().week() - self.start;
        if offset < 0 { offset += 7; }
        ((self.day + offset) as f64 / 7.0).ceil() as i32
    }

    /// 当年第几周。
    pub fn index_in_year(&self) -> i32 {
        let mut offset = Solar::from_ymd(self.year, 1, 1).unwrap().week() - self.start;
        if offset < 0 { offset += 7; }
        ((solar_util::days_in_year(self.year, self.month, self.day) + offset) as f64 / 7.0).ceil() as i32
    }

    /// 周第一天。
    pub fn first_day(&self) -> Solar {
        let c = Solar::from_ymd(self.year, self.month, self.day).unwrap();
        let mut prev = c.week() - self.start;
        if prev < 0 { prev += 7; }
        c.next_day(-prev)
    }

    /// 当周 7 天。
    pub fn days(&self) -> Vec<Solar> {
        let first = self.first_day();
        (0..7).map(|i| first.next_day(i)).collect()
    }

    /// 当周在当月内的第一天。
    pub fn first_day_in_month(&self) -> Option<Solar> {
        self.days().into_iter().find(|d| d.month() == self.month)
    }

    /// 当周属于当月的日期。
    pub fn days_in_month(&self) -> Vec<Solar> {
        self.days().into_iter().filter(|d| d.month() == self.month).collect()
    }

    /// 推进 / 回退若干周。
    pub fn next(&self, weeks: i32, separate_month: bool) -> Self {
        if weeks == 0 {
            return Self::from_ymd(self.year, self.month, self.day, self.start);
        }
        let mut c = Solar::from_ymd(self.year, self.month, self.day).unwrap();
        if separate_month {
            let mut n = weeks;
            let mut week = Self::from_ymd(c.year(), c.month(), c.day(), self.start);
            let mut month = self.month;
            let plus = n > 0;
            while n != 0 {
                c = if plus { c.next_day(7) } else { c.next_day(-7) };
                let mut w = Self::from_ymd(c.year(), c.month(), c.day(), self.start);
                let mut week_month = w.month();
                if month != week_month {
                    let index = w.index();
                    if plus {
                        if index == 1 {
                            let f = w.first_day();
                            w = Self::from_ymd(f.year(), f.month(), f.day(), self.start);
                            week_month = w.month;
                        } else {
                            c = Solar::from_ymd(w.year, w.month, 1).unwrap();
                            w = Self::from_ymd(c.year(), c.month(), c.day(), self.start);
                        }
                    } else if solar_util::weeks_of_month(w.year, w.month, self.start) == index {
                        let last = w.first_day().next_day(6);
                        w = Self::from_ymd(last.year(), last.month(), last.day(), self.start);
                        week_month = w.month;
                    } else {
                        let dm = solar_util::days_of_month(w.year, w.month);
                        c = Solar::from_ymd(w.year, w.month, dm).unwrap();
                        w = Self::from_ymd(c.year(), c.month(), c.day(), self.start);
                    }
                    month = week_month;
                }
                week = w;
                n -= if plus { 1 } else { -1 };
            }
            week
        } else {
            c = c.next_day(weeks * 7);
            Self::from_ymd(c.year(), c.month(), c.day(), self.start)
        }
    }
}

impl fmt::Display for SolarWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.month, self.index())
    }
}
