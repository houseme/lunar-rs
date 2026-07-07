//! 阳历月。对应 lunar-go `calendar/SolarMonth.go`。

use std::fmt;

use crate::event::{EventDayGroup, EventQuery, scan_event_days_in_range, scan_event_days_in_range_filtered};
use crate::multi_calendar::CalendarSpan;
use crate::solar::Solar;
use crate::solar_util;
use crate::solar_week::SolarWeek;

/// 阳历月。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct SolarMonth {
    year: i32,
    month: i32,
}

impl SolarMonth {
    pub const fn from_ym(year: i32, month: i32) -> Self {
        Self { year, month }
    }
    pub const fn year(&self) -> i32 {
        self.year
    }
    pub const fn month(&self) -> i32 {
        self.month
    }

    pub fn first_solar_day(&self) -> Solar {
        Solar::from_ymd(self.year, self.month, 1).unwrap()
    }

    pub fn last_solar_day(&self) -> Solar {
        Solar::from_ymd(self.year, self.month, solar_util::days_of_month(self.year, self.month)).unwrap()
    }

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
            if first.year() > self.year || first.month() > self.month {
                break;
            }
        }
        out
    }

    /// 当月按日分组的事件视图。
    pub fn event_days(&self) -> Vec<EventDayGroup> {
        let first = Solar::from_ymd(self.year, self.month, 1).unwrap();
        let last = Solar::from_ymd(self.year, self.month, solar_util::days_of_month(self.year, self.month)).unwrap();
        scan_event_days_in_range(first, last)
    }

    /// 当月按日分组的事件视图（带过滤条件）。
    pub fn find_event_days(&self, query: &EventQuery<'_>) -> Vec<EventDayGroup> {
        let first = Solar::from_ymd(self.year, self.month, 1).unwrap();
        let last = Solar::from_ymd(self.year, self.month, solar_util::days_of_month(self.year, self.month)).unwrap();
        scan_event_days_in_range_filtered(first, last, query)
    }

    /// 推进 / 回退若干月。
    pub const fn next(&self, months: i32) -> Self {
        let (n, m_abs) = if months < 0 { (-1, -months) } else { (1, months) };
        let mut y = self.year + (m_abs / 12) * n;
        let mut m = self.month + (m_abs % 12) * n;
        if m > 12 {
            m -= 12;
            y += 1;
        } else if m < 1 {
            m += 12;
            y -= 1;
        }
        Self::from_ym(y, m)
    }
}

impl fmt::Display for SolarMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.year, self.month)
    }
}

impl CalendarSpan for SolarMonth {
    fn first_solar_day(&self) -> Solar {
        SolarMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        SolarMonth::last_solar_day(self)
    }
}
