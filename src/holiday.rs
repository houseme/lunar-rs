//! 节假日对象。对应 lunar-go `HolidayUtil/Holiday.go`。

use std::fmt;

/// 一个法定节假日 / 调休日记录。
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
    #[inline]
    pub fn day(&self) -> &str { &self.day }
    #[inline]
    pub fn name(&self) -> &str { &self.name }
    #[inline]
    pub const fn is_work(&self) -> bool { self.work }
    #[inline]
    pub fn target(&self) -> &str { &self.target }
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
