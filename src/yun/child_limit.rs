//! 童限（出生到起运）provider 与 typed 信息。

use crate::Gender;
use crate::culture::{SixtyCycle, SixtyCycleYear};
use crate::lunar::Lunar;
use crate::lunar_util;
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

    pub const fn start_age(&self) -> i32 {
        1
    }

    pub fn end_age(&self) -> i32 {
        (self.end_solar().year() - self.start_solar().year()).max(1)
    }

    pub fn start_sixty_cycle_year(&self) -> SixtyCycleYear {
        sixty_cycle_year_from_solar_year(self.start_solar().year())
    }

    pub fn end_sixty_cycle_year(&self) -> SixtyCycleYear {
        sixty_cycle_year_from_solar_year(self.end_solar().year())
    }

    pub fn start_decade_fortune(&self) -> DecadeFortune<'a> {
        DecadeFortune::new(self.clone(), 0)
    }

    pub fn decade_fortune(&self) -> DecadeFortune<'a> {
        DecadeFortune::new(self.clone(), -1)
    }

    pub fn start_fortune(&self) -> Fortune<'a> {
        Fortune::new(self.clone(), 0)
    }
}

#[derive(Clone)]
pub struct DecadeFortune<'a> {
    child_limit: ChildLimit<'a>,
    index: i32,
}

impl<'a> DecadeFortune<'a> {
    pub const fn new(child_limit: ChildLimit<'a>, index: i32) -> Self {
        Self { child_limit, index }
    }

    pub fn next(&self, offset: i32) -> Self {
        Self::new(self.child_limit.clone(), self.index + offset)
    }

    pub const fn child_limit(&self) -> &ChildLimit<'a> {
        &self.child_limit
    }

    pub const fn index(&self) -> i32 {
        self.index
    }

    pub fn start_age(&self) -> i32 {
        self.child_limit.end_solar().year() - self.child_limit.start_solar().year() + 1 + self.index * 10
    }

    pub fn end_age(&self) -> i32 {
        self.start_age() + 9
    }

    pub fn start_year(&self) -> i32 {
        self.child_limit.end_solar().year() + self.index * 10
    }

    pub fn end_year(&self) -> i32 {
        self.start_year() + 9
    }

    pub fn sixty_cycle(&self) -> SixtyCycle {
        let month = SixtyCycle::from_name(&self.child_limit.lunar().month_in_gan_zhi_exact())
            .unwrap_or_else(|| SixtyCycle::from_index(0));
        let offset = (self.index + 1) as isize;
        month.next(if self.child_limit.is_forward() { offset } else { -offset })
    }

    pub fn name(&self) -> &'static str {
        self.sixty_cycle().name()
    }

    pub fn start_fortune(&self) -> Fortune<'a> {
        Fortune::new(self.child_limit.clone(), self.index * 10)
    }
}

#[derive(Clone)]
pub struct Fortune<'a> {
    child_limit: ChildLimit<'a>,
    index: i32,
}

impl<'a> Fortune<'a> {
    pub const fn new(child_limit: ChildLimit<'a>, index: i32) -> Self {
        Self { child_limit, index }
    }

    pub fn next(&self, offset: i32) -> Self {
        Self::new(self.child_limit.clone(), self.index + offset)
    }

    pub const fn child_limit(&self) -> &ChildLimit<'a> {
        &self.child_limit
    }

    pub const fn index(&self) -> i32 {
        self.index
    }

    pub fn age(&self) -> i32 {
        self.child_limit.end_solar().year() - self.child_limit.start_solar().year() + 1 + self.index
    }

    pub fn year(&self) -> i32 {
        self.child_limit.end_solar().year() + self.index
    }

    pub fn sixty_cycle_year(&self) -> SixtyCycleYear {
        sixty_cycle_year_from_solar_year(self.year())
    }

    pub fn sixty_cycle(&self) -> SixtyCycle {
        let hour = SixtyCycle::from_name(&self.child_limit.lunar().time_in_gan_zhi())
            .unwrap_or_else(|| SixtyCycle::from_index(0));
        let offset = self.age() as isize;
        hour.next(if self.child_limit.is_forward() { offset } else { -offset })
    }

    pub fn name(&self) -> &'static str {
        self.sixty_cycle().name()
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

fn sixty_cycle_year_from_solar_year(year: i32) -> SixtyCycleYear {
    let name = format!(
        "{}{}",
        lunar_util::tables::GAN[((year - 4).rem_euclid(10) + 1) as usize],
        lunar_util::tables::ZHI[((year - 4).rem_euclid(12) + 1) as usize]
    );
    let index = lunar_util::get_jia_zi_index(&name);
    SixtyCycleYear::new(SixtyCycle::from_index(index as usize))
}
