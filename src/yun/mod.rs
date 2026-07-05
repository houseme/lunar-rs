//! 运程（大运 / 流年 / 流月 / 小运）。对应 lunar-go `calendar/Yun.go`、`DaYun.go` 等。
//!
//! 每个结构体持有 `&Lunar` 并在构造期捕获所需标量（顺逆、起始年等），避免嵌套借用。

mod da_yun;
mod liu_nian;
mod liu_yue;
mod xiao_yun;

pub use da_yun::DaYun;
pub use liu_nian::LiuNian;
pub use liu_yue::LiuYue;
pub use xiao_yun::XiaoYun;

use crate::Gender;
use crate::lunar::Lunar;
use crate::lunar_util;
use crate::solar::Solar;

/// 大运（运程）。
pub struct Yun<'a> {
    gender: Gender,
    start_year: i32,
    start_month: i32,
    start_day: i32,
    start_hour: i32,
    forward: bool,
    lunar: &'a Lunar,
}

impl<'a> Yun<'a> {
    pub(crate) fn new(lunar: &'a Lunar, gender: Gender, sect: u8) -> Self {
        let yang = lunar.year_gan_index_exact() % 2 == 0;
        let man = gender == 1;
        let forward = (yang && man) || (!yang && !man);
        let mut yun = Self { gender, start_year: 0, start_month: 0, start_day: 0, start_hour: 0, forward, lunar };
        yun.compute_start(sect);
        yun
    }

    fn compute_start(&mut self, sect: u8) {
        let prev = self.lunar.prev_jie();
        let next = self.lunar.next_jie();
        let current = self.lunar.solar();
        let (start, end) =
            if self.forward { (current, next.unwrap().solar()) } else { (prev.unwrap().solar(), current) };
        let (mut year, mut month, mut day, mut hour) = (0_i32, 0_i32, 0_i32, 0_i32);
        if sect == 2 {
            let mut minutes = end.subtract_minute(&start);
            year = minutes / 4320;
            minutes -= year * 4320;
            month = minutes / 360;
            minutes -= month * 360;
            day = minutes / 12;
            minutes -= day * 12;
            hour = minutes * 2;
        } else {
            let end_time_zhi_index = if end.hour() != 23 {
                lunar_util::get_time_zhi_index(&format!("{:02}:{:02}", end.hour(), end.minute()))
            } else {
                11
            };
            let start_time_zhi_index = if start.hour() != 23 {
                lunar_util::get_time_zhi_index(&format!("{:02}:{:02}", start.hour(), start.minute()))
            } else {
                11
            };
            let mut hour_diff = end_time_zhi_index - start_time_zhi_index;
            let mut day_diff = end.subtract(&start);
            if hour_diff < 0 {
                hour_diff += 12;
                day_diff -= 1;
            }
            let month_diff = (hour_diff * 10 / 30) as i32;
            month = day_diff * 4 + month_diff;
            day = (hour_diff as i32) * 10 - month_diff * 30;
            year = month / 12;
            month -= year * 12;
        }
        self.start_year = year;
        self.start_month = month;
        self.start_day = day;
        self.start_hour = hour;
    }

    pub const fn gender(&self) -> Gender {
        self.gender
    }
    pub const fn start_year(&self) -> i32 {
        self.start_year
    }
    pub const fn start_month(&self) -> i32 {
        self.start_month
    }
    pub const fn start_day(&self) -> i32 {
        self.start_day
    }
    pub const fn start_hour(&self) -> i32 {
        self.start_hour
    }
    pub const fn is_forward(&self) -> bool {
        self.forward
    }
    pub const fn lunar(&self) -> &Lunar {
        self.lunar
    }

    pub fn start_solar(&self) -> Solar {
        let s = self.lunar.solar();
        s.next_year(self.start_year).next_month(self.start_month).next_day(self.start_day).next_hour(self.start_hour)
    }

    /// 获取 n 轮大运（默认 10）。
    pub fn da_yun(&self) -> Vec<DaYun<'a>> {
        self.da_yun_by(10)
    }
    pub fn da_yun_by(&self, n: usize) -> Vec<DaYun<'a>> {
        let start_year = self.start_solar().year();
        (0..n).map(|i| DaYun::new(self.lunar, start_year, i as i32, self.forward)).collect()
    }
}
