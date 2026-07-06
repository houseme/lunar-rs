//! 阳历年。对应 lunar-go `calendar/SolarYear.go`。

use std::fmt;

use crate::solar_month::SolarMonth;

const MONTH_IN_YEAR: i32 = 12;

/// 阳历年。
#[derive(Clone, Copy, Debug)]
pub struct SolarYear {
    year: i32,
}

impl SolarYear {
    pub const fn from_year(year: i32) -> Self {
        Self { year }
    }
    pub const fn year(&self) -> i32 {
        self.year
    }

    /// 当年 12 个月。
    pub fn months(&self) -> Vec<SolarMonth> {
        let first = SolarMonth::from_ym(self.year, 1);
        (0..MONTH_IN_YEAR).map(|i| first.next(i)).collect()
    }

    pub const fn next(&self, years: i32) -> Self {
        Self::from_year(self.year + years)
    }
}

impl fmt::Display for SolarYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.year)
    }
}
