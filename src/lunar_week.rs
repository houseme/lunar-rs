//! 农历周。对标 tyme4rs `LunarWeek`。

use std::fmt;

use crate::lunar::Lunar;
use crate::lunar_month::LunarMonth;
use crate::lunar_util;
use crate::unit::{WEEK_UNIT_NAMES, WeekUnit};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LunarWeek {
    year: i32,
    month: i32,
    index: usize,
    start: usize,
}

impl LunarWeek {
    pub fn from_ym(year: i32, month: i32, index: usize, start: usize) -> Option<Self> {
        WeekUnit::from_ym(year, month, index, start)?;
        if index >= Self::week_count(year, month, start)? {
            return None;
        }
        Some(Self { year, month, index, start })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn start(&self) -> usize {
        self.start
    }

    pub const fn name(&self) -> &'static str {
        WEEK_UNIT_NAMES[self.index]
    }

    pub fn week_unit(&self) -> Option<WeekUnit> {
        WeekUnit::from_ym(self.year, self.month, self.index, self.start)
    }

    pub fn lunar_month(&self) -> Option<LunarMonth> {
        LunarMonth::from_ym(self.year, self.month)
    }

    pub fn first_day(&self) -> Option<Lunar> {
        let first = Lunar::from_ymd(self.year, self.month, 1).ok()?;
        let offset = (first.week() - self.start as i32).rem_euclid(7);
        Some(first.next(self.index as i32 * 7 - offset))
    }

    pub fn days(&self) -> Option<Vec<Lunar>> {
        let first = self.first_day()?;
        Some((0..7).map(|offset| first.next(offset)).collect())
    }

    pub fn week_count(year: i32, month: i32, start: usize) -> Option<usize> {
        if start > 6 {
            return None;
        }
        let lunar_month = LunarMonth::from_ym(year, month)?;
        let first = Lunar::from_ymd(year, month, 1).ok()?.solar();
        let offset = (first.week() - start as i32).rem_euclid(7);
        let first_week_start = first.next_day(-offset);
        let last = Lunar::from_ymd(year, month, lunar_month.day_count()).ok()?.solar();
        Some((last.subtract(&first_week_start) / 7 + 1) as usize)
    }

    pub fn next(&self, offset: isize) -> Option<Self> {
        let mut target_index = self.index as isize + offset;
        let mut month = LunarMonth::from_ym(self.year, self.month)?;
        if offset > 0 {
            let mut week_count = Self::week_count(month.year(), month.month(), self.start)? as isize;
            while target_index >= week_count {
                target_index -= week_count;
                month = month.next(1)?;
                if month.first_solar_day().lunar().week() != self.start as i32 {
                    target_index += 1;
                }
                week_count = Self::week_count(month.year(), month.month(), self.start)? as isize;
            }
        } else {
            while target_index < 0 {
                if month.first_solar_day().lunar().week() != self.start as i32 {
                    target_index -= 1;
                }
                month = month.next(-1)?;
                target_index += Self::week_count(month.year(), month.month(), self.start)? as isize;
            }
        }
        Self::from_ym(month.year(), month.month(), target_index as usize, self.start)
    }
}

impl fmt::Display for LunarWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let month_name = lunar_util::tables::MONTH[self.month.unsigned_abs() as usize];
        let leap = if self.month < 0 { "闰" } else { "" };
        write!(f, "{}年{}{}月{}", self.year, leap, month_name, self.name())
    }
}
