//! 童限（出生到起运）provider 与 typed 信息。

use crate::Gender;
use crate::lunar::Lunar;
use crate::solar::Solar;
use crate::yun::Yun;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChildLimitInfo {
    start_solar: Solar,
    end_solar: Solar,
    year_count: i32,
    month_count: i32,
    day_count: i32,
    hour_count: i32,
    minute_count: i32,
}

impl ChildLimitInfo {
    pub const fn new(
        start_solar: Solar,
        end_solar: Solar,
        year_count: i32,
        month_count: i32,
        day_count: i32,
        hour_count: i32,
        minute_count: i32,
    ) -> Self {
        Self { start_solar, end_solar, year_count, month_count, day_count, hour_count, minute_count }
    }

    pub(crate) fn from_yun(yun: &Yun<'_>) -> Self {
        Self::new(
            yun.lunar().solar(),
            yun.start_solar(),
            yun.start_year(),
            yun.start_month(),
            yun.start_day(),
            yun.start_hour(),
            0,
        )
    }

    pub const fn start_solar(&self) -> Solar {
        self.start_solar
    }

    pub const fn end_solar(&self) -> Solar {
        self.end_solar
    }

    pub const fn year_count(&self) -> i32 {
        self.year_count
    }

    pub const fn month_count(&self) -> i32 {
        self.month_count
    }

    pub const fn day_count(&self) -> i32 {
        self.day_count
    }

    pub const fn hour_count(&self) -> i32 {
        self.hour_count
    }

    pub const fn minute_count(&self) -> i32 {
        self.minute_count
    }
}

#[derive(Clone)]
pub struct ChildLimit<'a> {
    lunar: &'a Lunar,
    gender: Gender,
    forward: bool,
    info: ChildLimitInfo,
}

impl<'a> ChildLimit<'a> {
    pub(crate) const fn new(lunar: &'a Lunar, gender: Gender, forward: bool, info: ChildLimitInfo) -> Self {
        Self { lunar, gender, forward, info }
    }

    pub const fn lunar(&self) -> &Lunar {
        self.lunar
    }

    pub const fn gender(&self) -> Gender {
        self.gender
    }

    pub const fn is_forward(&self) -> bool {
        self.forward
    }

    pub const fn info(&self) -> ChildLimitInfo {
        self.info
    }

    pub const fn start_solar(&self) -> Solar {
        self.info.start_solar()
    }

    pub const fn end_solar(&self) -> Solar {
        self.info.end_solar()
    }

    pub const fn year_count(&self) -> i32 {
        self.info.year_count()
    }

    pub const fn month_count(&self) -> i32 {
        self.info.month_count()
    }

    pub const fn day_count(&self) -> i32 {
        self.info.day_count()
    }

    pub const fn hour_count(&self) -> i32 {
        self.info.hour_count()
    }

    pub const fn minute_count(&self) -> i32 {
        self.info.minute_count()
    }
}

pub trait ChildLimitProvider {
    fn child_limit_info(&self, lunar: &Lunar, gender: Gender) -> ChildLimitInfo;

    fn child_limit<'a>(&self, lunar: &'a Lunar, gender: Gender) -> ChildLimit<'a> {
        let info = self.child_limit_info(lunar, gender);
        ChildLimit::new(lunar, gender, is_forward(lunar, gender), info)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DefaultChildLimitProvider;

impl DefaultChildLimitProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl ChildLimitProvider for DefaultChildLimitProvider {
    fn child_limit_info(&self, lunar: &Lunar, gender: Gender) -> ChildLimitInfo {
        let start = lunar.solar();
        let term = limit_term(lunar, gender);
        let mut seconds = ((term.julian_day() - start.julian_day()).abs() * 86_400.0).round() as i32;
        let year = seconds / 259_200;
        seconds %= 259_200;
        let month = seconds / 21_600;
        seconds %= 21_600;
        let day = seconds / 720;
        seconds %= 720;
        let hour = seconds / 30;
        seconds %= 30;
        let minute = seconds * 2;
        let end = add_limit_offset(start, year, month, day, hour, minute);
        ChildLimitInfo::new(start, end, year, month, day, hour, minute)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct China95ChildLimitProvider;

impl China95ChildLimitProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl ChildLimitProvider for China95ChildLimitProvider {
    fn child_limit_info(&self, lunar: &Lunar, gender: Gender) -> ChildLimitInfo {
        let start = lunar.solar();
        let term = limit_term(lunar, gender);
        let mut minutes = ((term.julian_day() - start.julian_day()).abs() * 1_440.0).round() as i32;
        let year = minutes / 4_320;
        minutes %= 4_320;
        let month = minutes / 360;
        minutes %= 360;
        let day = minutes / 12;
        let end = add_limit_offset(start, year, month, day, 0, 0);
        ChildLimitInfo::new(start, end, year, month, day, 0, 0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LunarSect1ChildLimitProvider;

impl LunarSect1ChildLimitProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl ChildLimitProvider for LunarSect1ChildLimitProvider {
    fn child_limit_info(&self, lunar: &Lunar, gender: Gender) -> ChildLimitInfo {
        Yun::new(lunar, gender, 1).child_limit_info()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LunarSect2ChildLimitProvider;

impl LunarSect2ChildLimitProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl ChildLimitProvider for LunarSect2ChildLimitProvider {
    fn child_limit_info(&self, lunar: &Lunar, gender: Gender) -> ChildLimitInfo {
        Yun::new(lunar, gender, 2).child_limit_info()
    }
}

fn is_forward(lunar: &Lunar, gender: Gender) -> bool {
    let yang = lunar.year_gan_index_exact() % 2 == 0;
    let man = gender == 1;
    (yang && man) || (!yang && !man)
}

fn limit_term(lunar: &Lunar, gender: Gender) -> Solar {
    if is_forward(lunar, gender) { lunar.next_jie().unwrap().solar() } else { lunar.prev_jie().unwrap().solar() }
}

fn add_limit_offset(start: Solar, years: i32, months: i32, days: i32, hours: i32, minutes: i32) -> Solar {
    add_minutes(start.next_year(years).next_month(months).next_day(days).next_hour(hours), minutes)
}

fn add_minutes(start: Solar, minutes: i32) -> Solar {
    let total = start.minute() + minutes;
    let hour_offset = total.div_euclid(60);
    let minute = total.rem_euclid(60);
    let shifted = start.next_hour(hour_offset);
    Solar::from_ymd_hms(shifted.year(), shifted.month(), shifted.day(), shifted.hour(), minute, shifted.second())
        .unwrap_or(shifted)
}
