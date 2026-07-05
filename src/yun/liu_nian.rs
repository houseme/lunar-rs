//! 流年（大运中的每一年）。

use crate::lunar::Lunar;
use crate::lunar_util;
use crate::yun::LiuYue;

/// 流年。
pub struct LiuNian<'a> {
    index: i32,
    year: i32,
    age: i32,
    da_yun_start_age: i32,
    da_yun_index: i32,
    lunar: &'a Lunar,
}

impl<'a> LiuNian<'a> {
    pub(crate) fn new(
        lunar: &'a Lunar,
        da_yun_start_year: i32,
        da_yun_start_age: i32,
        da_yun_index: i32,
        index: i32,
    ) -> Self {
        Self {
            index,
            year: da_yun_start_year + index,
            age: da_yun_start_age + index,
            da_yun_start_age,
            da_yun_index,
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
        let li_chun = self.lunar.jie_qi_table().get("立春").copied().unwrap_or(self.lunar.solar());
        let offset_base = lunar_util::get_jia_zi_index(&li_chun.lunar().year_in_gan_zhi_exact());
        let mut offset = offset_base + self.index as i64;
        if self.da_yun_index > 0 {
            offset += (self.da_yun_start_age - 1) as i64;
        }
        let size = lunar_util::tables::JIA_ZI.len() as i64;
        let offset = offset.rem_euclid(size);
        lunar_util::tables::JIA_ZI[offset as usize].to_string()
    }
    pub fn xun(&self) -> &'static str {
        lunar_util::get_xun(&self.gan_zhi())
    }
    pub fn xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.gan_zhi())
    }

    pub fn liu_yue(&self) -> Vec<LiuYue> {
        let gz = self.gan_zhi();
        (0..12).map(|i| LiuYue::new(&gz, i)).collect()
    }
}
