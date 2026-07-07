//! 节假日对象。对应 lunar-go `HolidayUtil/Holiday.go`。

use std::fmt;

use crate::event::{CalendarKind, Event, EventDetail, EventKind, EventSource, EventSourceId};
use crate::holiday_util;
use crate::solar::Solar;

/// 一个法定节假日 / 调休日记录。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Holiday {
    day: FixedYmd,
    name: Box<str>,
    work: bool,
    target: FixedYmd,
}

impl Holiday {
    pub(crate) fn new(day: &str, name: &str, work: bool, target: &str) -> Self {
        Self { day: FixedYmd::new(day), name: name.into(), work, target: FixedYmd::new(target) }
    }

    pub fn from_ymd(year: i32, month: i32, day: i32) -> Option<Self> {
        holiday_util::get_holiday_by_ymd(year, month, day)
    }

    #[inline]
    pub fn day(&self) -> &str {
        self.day.as_str()
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
        self.target.as_str()
    }

    pub fn get_day(&self) -> Solar {
        parse_ymd(self.day())
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_target(&self) -> Solar {
        parse_ymd(self.target())
    }

    pub fn to_event(&self, solar: Solar, calendar_kind: CalendarKind) -> Event {
        Event::with_meta(
            EventKind::Holiday,
            calendar_kind,
            EventSource::HolidayData,
            self.name().to_string(),
            solar,
            Some(EventDetail::Holiday { work: self.work, target: self.get_target() }),
            20,
            Some(EventSourceId::Holiday { day: self.get_day() }),
            !self.work,
            true,
            [
                "holiday",
                "observance",
                if self.work { "workday_remap" } else { "day_off" },
                match calendar_kind {
                    CalendarKind::Solar => "solar",
                    CalendarKind::Lunar => "lunar",
                    CalendarKind::Foto => "foto",
                    CalendarKind::Tao => "tao",
                },
            ],
        )
    }
}

impl fmt::Display for Holiday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let work_desc = if self.work { "调休" } else { "" };
        write!(f, "{} {}{} {}", self.day(), self.name(), work_desc, self.target())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
struct FixedYmd {
    bytes: [u8; 10],
}

impl FixedYmd {
    fn new(input: &str) -> Self {
        let normalized = normalize_ymd(input);
        let mut bytes = [b'0'; 10];
        bytes.copy_from_slice(normalized.as_bytes());
        Self { bytes }
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap_or("0000-00-00")
    }
}

fn normalize_ymd(s: &str) -> String {
    if s.len() >= 10 && s.as_bytes().get(4) == Some(&b'-') && s.as_bytes().get(7) == Some(&b'-') {
        return s[0..10].to_string();
    }
    if s.len() >= 8 {
        let mut normalized = String::with_capacity(10);
        normalized.push_str(&s[0..4]);
        normalized.push('-');
        normalized.push_str(&s[4..6]);
        normalized.push('-');
        normalized.push_str(&s[6..8]);
        return normalized;
    }
    "0000-00-00".to_string()
}

fn parse_ymd(s: &str) -> Solar {
    let (year, month, day) = parse_ymd_parts(s).unwrap_or((1, 1, 1));
    Solar::from_ymd(year, month, day).unwrap_or(Solar { year: 1, month: 1, day: 1, hour: 0, minute: 0, second: 0 })
}

fn parse_ymd_parts(s: &str) -> Option<(i32, i32, i32)> {
    let bytes = s.as_bytes();
    if bytes.len() >= 10 && bytes[4] == b'-' && bytes[7] == b'-' {
        let year = s[0..4].parse::<i32>().ok()?;
        let month = s[5..7].parse::<i32>().ok()?;
        let day = s[8..10].parse::<i32>().ok()?;
        return Some((year, month, day));
    }
    if bytes.len() >= 8 && bytes[0..8].iter().all(u8::is_ascii_digit) {
        let year = s[0..4].parse::<i32>().ok()?;
        let month = s[4..6].parse::<i32>().ok()?;
        let day = s[6..8].parse::<i32>().ok()?;
        return Some((year, month, day));
    }
    None
}
