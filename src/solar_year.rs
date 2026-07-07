//! 阳历年。对应 lunar-go `calendar/SolarYear.go`。

use std::fmt;

use crate::multi_calendar::CalendarSpan;
use crate::solar::Solar;
use crate::solar_month::SolarMonth;

const MONTH_IN_YEAR: i32 = 12;

/// 阳历年。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn first_solar_day(&self) -> Solar {
        Solar::from_ymd(self.year, 1, 1).unwrap()
    }

    pub fn last_solar_day(&self) -> Solar {
        Solar::from_ymd(self.year, 12, 31).unwrap()
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

impl CalendarSpan for SolarYear {
    fn first_solar_day(&self) -> Solar {
        SolarYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        SolarYear::last_solar_day(self)
    }
}
