//! 阳历年。对应 lunar-go `calendar/SolarYear.go`。

use std::fmt;

use crate::multi_calendar::CalendarSpan;
use crate::rab_byung::RabByungYear;
use crate::solar::Solar;
use crate::solar_half_year::SolarHalfYear;
use crate::solar_month::SolarMonth;
use crate::solar_season::SolarSeason;
use crate::solar_util;

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

    pub const fn get_year(&self) -> i32 {
        self.year()
    }

    pub fn get_day_count(&self) -> i32 {
        if self.year == 1582 {
            355
        } else if self.is_leap() {
            366
        } else {
            365
        }
    }

    pub const fn is_leap(&self) -> bool {
        solar_util::is_leap_year(self.year)
    }

    /// 当年 12 个月。
    pub fn months(&self) -> Vec<SolarMonth> {
        let first = SolarMonth::from_ym(self.year, 1);
        (0..MONTH_IN_YEAR).map(|i| first.next(i)).collect()
    }

    pub fn get_months(&self) -> Vec<SolarMonth> {
        self.months()
    }

    pub fn get_seasons(&self) -> Vec<SolarSeason> {
        (0..4).map(|index| SolarSeason::from_index(self.year, index)).collect()
    }

    pub fn get_half_years(&self) -> Vec<SolarHalfYear> {
        (0..2).map(|index| SolarHalfYear::from_index(self.year, index)).collect()
    }

    pub fn get_rab_byung_year(&self) -> Result<RabByungYear, crate::LunarError> {
        RabByungYear::from_year(self.year)
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
