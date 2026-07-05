//! 错误类型。
//!
//! Go 参考实现在非法日期处直接 `panic`，Rust 版改为返回 [`LunarError`]，
//! 让调用方自行决定处理方式（回退、日志、终止等）。

use std::fmt;

/// lunar-rs 的统一错误类型。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LunarError {
    /// 非法阳历字段（年 / 月 / 日 / 时 / 分 / 秒越界）。
    InvalidSolar { year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32 },
    /// 1582 历法改革中被删除的 10 天（1582-10-05 ~ 1582-10-14）。
    GregorianGap { year: i32, month: i32, day: i32 },
    /// 非法农历字段（年 / 月 / 日）。
    InvalidLunar { year: i32, month: i32, day: i32 },
    /// 农历日超出当月天数。
    LunarDayOverflow { year: i32, month: i32, day: i32, max: i32 },
    /// 该年无此闰月（例如指定了闰四月但当年不闰四月）。
    LeapMonthAbsent { year: i32, month: i32 },
    /// 输入字符串无法解析。
    Parse(String),
}

impl fmt::Display for LunarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSolar { year, month, day, hour, minute, second } => {
                write!(f, "invalid solar date: {year}-{month}-{day} {hour}:{minute}:{second}")
            }
            Self::GregorianGap { year, month, day } => {
                write!(f, "date {year}-{month}-{day} does not exist (Gregorian reform gap)")
            }
            Self::InvalidLunar { year, month, day } => {
                write!(f, "invalid lunar date: {year}-{month}-{day}")
            }
            Self::LunarDayOverflow { year, month, day, max } => {
                write!(f, "lunar day {day} out of range: only {max} days in lunar {year}-{month}")
            }
            Self::LeapMonthAbsent { year, month } => {
                write!(f, "leap month {month} does not exist in lunar year {year}")
            }
            Self::Parse(s) => write!(f, "parse error: {s}"),
        }
    }
}

impl std::error::Error for LunarError {}
