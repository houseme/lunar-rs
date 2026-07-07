//! 节假日对象。对应 lunar-go `HolidayUtil/Holiday.go`。

use std::fmt;

use crate::event::{CalendarKind, Event, EventKind, EventSource};
use crate::holiday_util;
use crate::solar::Solar;

/// 一个法定节假日 / 调休日记录。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Holiday {
    day: String,
    name: String,
    work: bool,
    target: String,
}

impl Holiday {
    pub(crate) fn new(day: &str, name: &str, work: bool, target: &str) -> Self {
        Self { day: fmt_dash(day), name: name.to_string(), work, target: fmt_dash(target) }
    }

    pub fn from_ymd(year: i32, month: i32, day: i32) -> Option<Self> {
        holiday_util::get_holiday_by_ymd(year, month, day)
    }

    #[inline]
    pub fn day(&self) -> &str {
        &self.day
    }
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub const fn is_work(&self) -> bool {
        self.work
    }
    #[inline]
    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn get_day(&self) -> Solar {
        parse_ymd(&self.day)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_target(&self) -> Solar {
        parse_ymd(&self.target)
    }

    pub fn to_event(&self, solar: Solar, calendar_kind: CalendarKind) -> Event {
        let detail = format!("work={} target={}", self.work, self.target);
        Event::with_meta(
            EventKind::Holiday,
            calendar_kind,
            EventSource::HolidayData,
            self.name(),
            solar,
            Some(detail),
            20,
            Some(format!("holiday:{}:{}", self.day(), self.name())),
            !self.work,
            true,
            vec![
                "holiday".to_string(),
                "observance".to_string(),
                if self.work { "workday_remap".to_string() } else { "day_off".to_string() },
                match calendar_kind {
                    CalendarKind::Solar => "solar",
                    CalendarKind::Lunar => "lunar",
                    CalendarKind::Foto => "foto",
                    CalendarKind::Tao => "tao",
                }
                .to_string(),
            ],
        )
    }
}

impl fmt::Display for Holiday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let work_desc = if self.work { "调休" } else { "" };
        write!(f, "{} {}{} {}", self.day, self.name, work_desc, self.target)
    }
}

/// 把 `YYYYMMDD` 格式化为 `YYYY-MM-DD`（已含 `-` 则原样返回）。
fn fmt_dash(s: &str) -> String {
    if s.contains('-') || s.len() < 8 {
        return s.to_string();
    }
    let b = s.as_bytes();
    format!(
        "{}-{}-{}",
        std::str::from_utf8(&b[0..4]).unwrap_or(""),
        std::str::from_utf8(&b[4..6]).unwrap_or(""),
        std::str::from_utf8(&b[6..8]).unwrap_or(""),
    )
}

fn parse_ymd(s: &str) -> Solar {
    let normalized = fmt_dash(s);
    let mut parts = normalized.split('-');
    let year = parts.next().and_then(|value| value.parse::<i32>().ok()).unwrap_or(1);
    let month = parts.next().and_then(|value| value.parse::<i32>().ok()).unwrap_or(1);
    let day = parts.next().and_then(|value| value.parse::<i32>().ok()).unwrap_or(1);
    Solar::from_ymd(year, month, day).unwrap_or(Solar { year: 1, month: 1, day: 1, hour: 0, minute: 0, second: 0 })
}
