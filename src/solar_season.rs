//! 阳历季度。对应 lunar-go `calendar/SolarSeason.go`。

use std::fmt;

use crate::solar_month::SolarMonth;

const MONTH_IN_SEASON: i32 = 3;

/// 阳历季度。
#[derive(Clone, Copy, Debug)]
pub struct SolarSeason {
    year: i32,
    month: i32,
}

impl SolarSeason {
    pub const fn from_ym(year: i32, month: i32) -> Self {
        Self { year, month }
    }
    pub const fn year(&self) -> i32 {
        self.year
    }
    pub const fn month(&self) -> i32 {
        self.month
    }

    /// 第几季度（1..4）。
    pub fn index(&self) -> i32 {
        (f64::from(self.month) / f64::from(MONTH_IN_SEASON)).ceil() as i32
    }
    pub fn months(&self) -> Vec<SolarMonth> {
        let index = self.index() - 1;
        (0..MONTH_IN_SEASON).map(|i| SolarMonth::from_ym(self.year, MONTH_IN_SEASON * index + i + 1)).collect()
    }
    pub const fn next(&self, seasons: i32) -> Self {
        let m = SolarMonth::from_ym(self.year, self.month).next(MONTH_IN_SEASON * seasons);
        Self::from_ym(m.year(), m.month())
    }
}

impl fmt::Display for SolarSeason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.year, self.index())
    }
}
