//! 时辰（两小时为一时辰）。对应 lunar-go `calendar/LunarTime.go`。

use crate::lunar::Lunar;
use crate::lunar_util;
use crate::nine_star::NineStar;

/// 时辰。借用其所属的 [`Lunar`]。
pub struct LunarTime<'a> {
    gan_index: i64,
    zhi_index: i64,
    lunar: &'a Lunar,
}

impl<'a> LunarTime<'a> {
    pub(crate) fn from_lunar(lunar: &'a Lunar) -> Self {
        let hm = format!("{:02}:{:02}", lunar.hour(), lunar.minute());
        let zhi_index = lunar_util::get_time_zhi_index(&hm);
        let gan_index = (lunar.day_gan_index_exact() % 5 * 2 + zhi_index) % 10;
        Self { gan_index, zhi_index, lunar }
    }

    pub const fn gan_index(&self) -> i64 {
        self.gan_index
    }
    pub const fn zhi_index(&self) -> i64 {
        self.zhi_index
    }

    pub fn gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.gan_index + 1) as usize]
    }
    pub fn zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.zhi_index + 1) as usize]
    }
    pub fn gan_zhi(&self) -> String {
        format!("{}{}", self.gan(), self.zhi())
    }
    pub fn sheng_xiao(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.zhi_index + 1) as usize]
    }

    pub fn position_xi(&self) -> &'static str {
        lunar_util::tables::POSITION_XI[(self.gan_index + 1) as usize]
    }
    pub fn position_xi_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_xi())
    }
    pub fn position_yang_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YANG_GUI[(self.gan_index + 1) as usize]
    }
    pub fn position_yang_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_yang_gui())
    }
    pub fn position_yin_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YIN_GUI[(self.gan_index + 1) as usize]
    }
    pub fn position_yin_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_yin_gui())
    }
    pub fn position_fu(&self) -> &'static str {
        self.position_fu_by_sect(2)
    }
    pub fn position_fu_by_sect(&self, sect: u8) -> &'static str {
        let offset = (self.gan_index + 1) as usize;
        if sect == 1 { lunar_util::tables::POSITION_FU[offset] } else { lunar_util::tables::POSITION_FU_2[offset] }
    }
    pub fn position_fu_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_fu_by_sect(2))
    }
    pub fn position_cai(&self) -> &'static str {
        lunar_util::tables::POSITION_CAI[(self.gan_index + 1) as usize]
    }
    pub fn position_cai_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_cai())
    }

    pub fn nayin(&self) -> &'static str {
        lunar_util::nayin(&self.gan_zhi())
    }

    pub fn tian_shen(&self) -> &'static str {
        let off = lunar_util::ZHI_TIAN_SHEN_OFFSET
            [lunar_util::find(self.lunar.day_zhi_exact(), lunar_util::tables::ZHI, 0) as usize];
        lunar_util::tables::TIAN_SHEN[((self.zhi_index + off) % 12 + 1) as usize]
    }
    pub fn tian_shen_type(&self) -> &'static str {
        lunar_util::tian_shen_type(self.tian_shen())
    }
    pub fn tian_shen_luck(&self) -> &'static str {
        lunar_util::tian_shen_type_luck(self.tian_shen_type())
    }

    pub fn chong(&self) -> &'static str {
        lunar_util::tables::CHONG[self.zhi_index as usize]
    }
    pub fn chong_gan(&self) -> &'static str {
        lunar_util::tables::CHONG_GAN[self.gan_index as usize]
    }
    pub fn chong_gan_tie(&self) -> &'static str {
        lunar_util::tables::CHONG_GAN_TIE[self.gan_index as usize]
    }
    pub fn sha(&self) -> &'static str {
        lunar_util::sha(self.zhi())
    }
    pub fn chong_sheng_xiao(&self) -> &'static str {
        let chong = self.chong();
        for (i, v) in lunar_util::tables::ZHI.iter().enumerate() {
            if *v == chong {
                return lunar_util::tables::SHENG_XIAO[i];
            }
        }
        ""
    }
    pub fn chong_desc(&self) -> String {
        format!("({}{}){}", self.chong_gan(), self.chong(), self.chong_sheng_xiao())
    }

    pub fn yi(&self) -> Vec<&'static str> {
        lunar_util::get_time_yi(&self.lunar.day_in_gan_zhi_exact(), &self.gan_zhi())
    }
    pub fn ji(&self) -> Vec<&'static str> {
        lunar_util::get_time_ji(&self.lunar.day_in_gan_zhi_exact(), &self.gan_zhi())
    }

    pub fn nine_star(&self) -> NineStar {
        let solar_ymd = self.lunar.solar().to_ymd();
        let jq = self.lunar.jie_qi_table();
        let dong_zhi = jq.get("冬至").copied().unwrap_or(self.lunar.solar());
        let xia_zhi = jq.get("夏至").copied().unwrap_or(self.lunar.solar());
        let mut asc = false;
        if solar_ymd >= dong_zhi.to_ymd() && solar_ymd < xia_zhi.to_ymd() {
            asc = true;
        }
        let mut start = if asc { 7_i64 } else { 3_i64 };
        let day_zhi = self.lunar.day_zhi();
        if "子午卯酉".contains(day_zhi) {
            start = if asc { 1 } else { 9 };
        } else if "辰戌丑未".contains(day_zhi) {
            start = if asc { 4 } else { 6 };
        }
        let mut index = if asc { start + self.zhi_index - 1 } else { start - self.zhi_index - 1 };
        if index > 8 {
            index -= 9;
        }
        if index < 0 {
            index += 9;
        }
        NineStar::from_index(index)
    }

    pub fn xun(&self) -> &'static str {
        lunar_util::get_xun(&self.gan_zhi())
    }
    pub fn xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.gan_zhi())
    }

    /// 当前时辰最早时分。
    pub fn min_hm(&self) -> String {
        let mut hour = self.lunar.hour();
        if hour < 1 {
            return "00:00".to_string();
        }
        if hour > 22 {
            return "23:00".to_string();
        }
        if hour % 2 == 0 {
            hour -= 1;
        }
        format!("{hour:02}:00")
    }
    /// 当前时辰最晚时分。
    pub fn max_hm(&self) -> String {
        let mut hour = self.lunar.hour();
        if hour < 1 {
            return "00:59".to_string();
        }
        if hour > 22 {
            return "23:59".to_string();
        }
        if hour % 2 != 0 {
            hour += 1;
        }
        format!("{hour:02}:59")
    }
}
