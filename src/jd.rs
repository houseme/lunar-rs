//! Julian day compatibility object.

use std::fmt;

use crate::{Solar, Week};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JulianDay {
    day: f64,
}

impl JulianDay {
    pub const fn from_julian_day(day: f64) -> Self {
        Self { day }
    }

    pub fn from_ymd_hms(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Option<Self> {
        Solar::from_ymd_hms(year, month, day, hour, minute, second)
            .ok()
            .map(|solar| Self::from_julian_day(solar.julian_day()))
    }

    pub const fn day(&self) -> f64 {
        self.day
    }

    pub const fn get_day(&self) -> f64 {
        self.day()
    }

    pub fn week(&self) -> Week {
        Week::from_index(((self.day + 0.5) as i64 + 7_000_001).rem_euclid(7) as usize)
    }

    pub fn solar_time(&self) -> Solar {
        Solar::from_julian_day(self.day)
    }

    pub fn get_solar_time(&self) -> Solar {
        self.solar_time()
    }

    pub fn solar_day(&self) -> Solar {
        let solar = self.solar_time();
        Solar::from_ymd(solar.year(), solar.month(), solar.day()).unwrap_or(solar)
    }

    pub fn get_solar_day(&self) -> Solar {
        self.solar_day()
    }

    pub fn next(&self, days: i32) -> Self {
        Self::from_julian_day(self.day + f64::from(days))
    }
}

impl fmt::Display for JulianDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.day)
    }
}
