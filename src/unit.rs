//! Calendar unit primitives compatible with tyme4rs unit objects.

pub const WEEK_UNIT_NAMES: [&str; 6] = ["第一周", "第二周", "第三周", "第四周", "第五周", "第六周"];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct YearUnit {
    year: i32,
}

impl YearUnit {
    pub const fn new(year: i32) -> Self {
        Self { year }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn compare_index(&self) -> i64 {
        self.year as i64 * 10_000
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MonthUnit {
    year: i32,
    month: i32,
}

impl MonthUnit {
    pub const fn new(year: i32, month: i32) -> Self {
        Self { year, month }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn year_unit(&self) -> YearUnit {
        YearUnit::new(self.year)
    }

    pub const fn compare_index(&self) -> i64 {
        let month_index = if self.month > 0 { self.month * 2 } else { -self.month * 2 + 1 };
        self.year_unit().compare_index() + month_index as i64 * 100
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DayUnit {
    year: i32,
    month: i32,
    day: i32,
}

impl DayUnit {
    pub const fn new(year: i32, month: i32, day: i32) -> Self {
        Self { year, month, day }
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub const fn month_unit(&self) -> MonthUnit {
        MonthUnit::new(self.year, self.month)
    }

    pub const fn compare_index(&self) -> i64 {
        self.month_unit().compare_index() + self.day as i64
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SecondUnit {
    year: i32,
    month: i32,
    day: i32,
    hour: u8,
    minute: u8,
    second: u8,
}

impl SecondUnit {
    pub const fn new(year: i32, month: i32, day: i32, hour: u8, minute: u8, second: u8) -> Self {
        Self { year, month, day, hour, minute, second }
    }

    pub const fn from_ymd_hms(year: i32, month: i32, day: i32, hour: u8, minute: u8, second: u8) -> Option<Self> {
        if hour > 23 || minute > 59 || second > 59 {
            return None;
        }
        Some(Self::new(year, month, day, hour, minute, second))
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub const fn hour(&self) -> u8 {
        self.hour
    }

    pub const fn minute(&self) -> u8 {
        self.minute
    }

    pub const fn second(&self) -> u8 {
        self.second
    }

    pub const fn day_unit(&self) -> DayUnit {
        DayUnit::new(self.year, self.month, self.day)
    }

    pub const fn seconds_in_day(&self) -> u32 {
        self.hour as u32 * 3_600 + self.minute as u32 * 60 + self.second as u32
    }

    pub const fn compare_index(&self) -> i64 {
        self.day_unit().compare_index() * 86_400 + self.seconds_in_day() as i64
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WeekUnit {
    year: i32,
    month: i32,
    index: usize,
    start: usize,
}

impl WeekUnit {
    pub const fn from_ym(year: i32, month: i32, index: usize, start: usize) -> Option<Self> {
        if index >= WEEK_UNIT_NAMES.len() || start > 6 {
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

    pub const fn month_unit(&self) -> MonthUnit {
        MonthUnit::new(self.year, self.month)
    }
}
