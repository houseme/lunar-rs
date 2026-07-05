//! 流月（流年中的每一月）。

use crate::lunar_util;

/// 流月（拥有自己的数据，无借用）。
pub struct LiuYue {
    index: i32,
    year_gan_zhi: String,
}

impl LiuYue {
    pub(crate) fn new(year_gan_zhi: &str, index: i32) -> Self {
        Self { index, year_gan_zhi: year_gan_zhi.to_string() }
    }

    pub const fn index(&self) -> i32 {
        self.index
    }

    pub fn month_in_chinese(&self) -> &'static str {
        lunar_util::tables::MONTH[(self.index + 1) as usize]
    }

    /// 《五虎遁》月干支。
    pub fn gan_zhi(&self) -> String {
        let year_gan = self.year_gan_zhi.chars().next().unwrap_or('甲').to_string();
        let offset = match year_gan.as_str() {
            "甲" | "己" => 2_i64,
            "乙" | "庚" => 4,
            "丙" | "辛" => 6,
            "丁" | "壬" => 8,
            _ => 0,
        };
        let i = self.index as i64;
        let gan = lunar_util::tables::GAN[((i + offset) % 10 + 1) as usize];
        let zhi = lunar_util::tables::ZHI[((i + lunar_util::BASE_MONTH_ZHI_INDEX) % 12 + 1) as usize];
        format!("{gan}{zhi}")
    }
    pub fn xun(&self) -> &'static str {
        lunar_util::get_xun(&self.gan_zhi())
    }
    pub fn xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.gan_zhi())
    }
}
