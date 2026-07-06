//! 小运（从时柱起，每年进一格）。

use crate::lunar::Lunar;
use crate::lunar_util;

/// 小运。
pub struct XiaoYun<'a> {
    index: i32,
    year: i32,
    age: i32,
    da_yun_start_age: i32,
    da_yun_index: i32,
    forward: bool,
    lunar: &'a Lunar,
}

impl<'a> XiaoYun<'a> {
    pub(crate) const fn new(
        lunar: &'a Lunar,
        da_yun_start_year: i32,
        da_yun_start_age: i32,
        da_yun_index: i32,
        index: i32,
        forward: bool,
    ) -> Self {
        Self {
            index,
            year: da_yun_start_year + index,
            age: da_yun_start_age + index,
            da_yun_start_age,
            da_yun_index,
            forward,
            lunar,
        }
    }

    pub const fn index(&self) -> i32 {
        self.index
    }
    pub const fn year(&self) -> i32 {
        self.year
    }
    pub const fn age(&self) -> i32 {
        self.age
    }

    pub fn gan_zhi(&self) -> String {
        let mut offset = lunar_util::get_jia_zi_index(&self.lunar.time_in_gan_zhi());
        let mut add = i64::from(self.index) + 1;
        if self.da_yun_index > 0 {
            add += i64::from(self.da_yun_start_age - 1);
        }
        if self.forward {
            offset += add;
        } else {
            offset -= add;
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
}
