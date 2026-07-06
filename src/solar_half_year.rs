//! 阳历半年。对应 lunar-go `calendar/SolarHalfYear.go`。

use std::fmt;

use crate::solar_month::SolarMonth;

const MONTH_IN_HALF_YEAR: i32 = 6;

/// 阳历半年。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct SolarHalfYear {
    year: i32,
    month: i32,
}

impl SolarHalfYear {
    pub const fn from_ym(year: i32, month: i32) -> Self {
        Self { year, month }
    }
    pub const fn year(&self) -> i32 {
        self.year
    }
    pub const fn month(&self) -> i32 {
        self.month
    }

    /// 上 / 下半年（1 / 2）。
    pub fn index(&self) -> i32 {
        (f64::from(self.month) / f64::from(MONTH_IN_HALF_YEAR)).ceil() as i32
    }
    pub fn months(&self) -> Vec<SolarMonth> {
        let index = self.index() - 1;
        (0..MONTH_IN_HALF_YEAR).map(|i| SolarMonth::from_ym(self.year, MONTH_IN_HALF_YEAR * index + i + 1)).collect()
    }
    pub const fn next(&self, half_years: i32) -> Self {
        let m = SolarMonth::from_ym(self.year, self.month).next(MONTH_IN_HALF_YEAR * half_years);
        Self::from_ym(m.year(), m.month())
    }
}

impl fmt::Display for SolarHalfYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.year, self.index())
    }
}
