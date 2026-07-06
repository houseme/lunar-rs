//! 大运（每 10 年一轮）。

use crate::lunar::Lunar;
use crate::lunar_util;
use crate::yun::{LiuNian, XiaoYun};

/// 一轮大运。
pub struct DaYun<'a> {
    start_year: i32,
    end_year: i32,
    start_age: i32,
    end_age: i32,
    index: i32,
    forward: bool,
    lunar: &'a Lunar,
}

impl<'a> DaYun<'a> {
    pub(crate) const fn new(lunar: &'a Lunar, yun_start_year: i32, index: i32, forward: bool) -> Self {
        let birth_year = lunar.solar().year();
        let year = yun_start_year;
        let (start_year, start_age, end_year, end_age);
        if index < 1 {
            start_year = birth_year;
            start_age = 1;
            end_year = year - 1;
            end_age = year - birth_year;
        } else {
            let add = (index - 1) * 10;
            start_year = year + add;
            start_age = start_year - birth_year + 1;
            end_year = start_year + 9;
            end_age = start_age + 9;
        }
        Self { start_year, end_year, start_age, end_age, index, forward, lunar }
    }

    pub const fn start_year(&self) -> i32 {
        self.start_year
    }
    pub const fn end_year(&self) -> i32 {
        self.end_year
    }
    pub const fn start_age(&self) -> i32 {
        self.start_age
    }
    pub const fn end_age(&self) -> i32 {
        self.end_age
    }
    pub const fn index(&self) -> i32 {
        self.index
    }
    pub const fn lunar(&self) -> &Lunar {
        self.lunar
    }

    pub fn gan_zhi(&self) -> String {
        if self.index < 1 {
            return String::new();
        }
        let mut offset = lunar_util::get_jia_zi_index(&self.lunar.month_in_gan_zhi_exact());
        if self.forward {
            offset += i64::from(self.index);
        } else {
            offset -= i64::from(self.index);
        }
        let size = lunar_util::tables::JIA_ZI.len() as i64;
        offset = offset.rem_euclid(size);
        lunar_util::tables::JIA_ZI[offset as usize].to_string()
    }
    pub fn xun(&self) -> &'static str {
        lunar_util::get_xun(&self.gan_zhi())
    }
    pub fn xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.gan_zhi())
    }

    pub fn liu_nian(&self) -> Vec<LiuNian<'a>> {
        self.liu_nian_by(10)
    }
    pub fn liu_nian_by(&self, mut n: i32) -> Vec<LiuNian<'a>> {
        if self.index < 1 {
            n = self.end_year - self.start_year + 1;
        }
        (0..n).map(|i| LiuNian::new(self.lunar, self.start_year, self.start_age, self.index, i)).collect()
    }
    pub fn xiao_yun(&self) -> Vec<XiaoYun<'a>> {
        self.xiao_yun_by(10)
    }
    pub fn xiao_yun_by(&self, mut n: i32) -> Vec<XiaoYun<'a>> {
        if self.index < 1 {
            n = self.end_year - self.start_year + 1;
        }
        (0..n).map(|i| XiaoYun::new(self.lunar, self.start_year, self.start_age, self.index, i, self.forward)).collect()
    }
}
