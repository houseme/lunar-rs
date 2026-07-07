//! 八字（四柱）。对应 lunar-go `calendar/EightChar.go`。

use crate::Gender;
use crate::culture::{
    SixtyCycle, SixtyCycleDay, SixtyCycleHour, SixtyCycleMonth, SixtyCycleYear, Terrain, ThreePillars,
};
use crate::lunar::Lunar;
use crate::lunar_util;
use crate::yun::{ChildLimit, ChildLimitProvider, DefaultChildLimitProvider, Yun};

const MONTH_ZHI: &[&str; 13] = &["", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];
const CHANG_SHENG: &[&str; 12] = &["长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养"];

fn chang_sheng_offset(gan: &str) -> i64 {
    match gan {
        "甲" => 1,
        "丙" | "戊" => 10,
        "庚" => 7,
        "壬" => 4,
        "乙" => 6,
        "丁" | "己" => 9,
        "癸" => 3,
        _ => 0,
    }
}

/// 八字（四柱：年 / 月 / 日 / 时）。借用底层 [`Lunar`]。
pub struct EightChar<'a> {
    sect: u8,
    lunar: &'a Lunar,
}

pub trait EightCharProvider {
    fn sect(&self, lunar: &Lunar) -> u8;

    fn eight_char<'a>(&self, lunar: &'a Lunar) -> EightChar<'a> {
        let mut eight_char = EightChar::from_lunar(lunar);
        eight_char.set_sect(self.sect(lunar));
        eight_char
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DefaultEightCharProvider;

impl DefaultEightCharProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl EightCharProvider for DefaultEightCharProvider {
    fn sect(&self, _lunar: &Lunar) -> u8 {
        2
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LunarSect1EightCharProvider;

impl LunarSect1EightCharProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl EightCharProvider for LunarSect1EightCharProvider {
    fn sect(&self, _lunar: &Lunar) -> u8 {
        1
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LunarSect2EightCharProvider;

impl LunarSect2EightCharProvider {
    pub const fn new() -> Self {
        Self
    }
}

impl EightCharProvider for LunarSect2EightCharProvider {
    fn sect(&self, _lunar: &Lunar) -> u8 {
        2
    }
}

impl<'a> EightChar<'a> {
    pub(crate) const fn from_lunar(lunar: &'a Lunar) -> Self {
        Self { sect: 2, lunar }
    }

    pub const fn sect(&self) -> u8 {
        self.sect
    }
    pub const fn set_sect(&mut self, sect: u8) {
        self.sect = if sect == 1 { 1 } else { 2 };
    }
    pub const fn lunar(&self) -> &Lunar {
        self.lunar
    }

    fn shi_shen_zhi(&self, zhi: &str) -> Vec<&'static str> {
        let day_gan = self.day_gan();
        lunar_util::zhi_hide_gan(zhi).iter().map(|h| lunar_util::shi_shen(&format!("{day_gan}{h}"))).collect()
    }

    pub const fn day_gan_index(&self) -> i64 {
        if self.sect == 2 { self.lunar.day_gan_index_exact2() } else { self.lunar.day_gan_index_exact() }
    }
    pub const fn day_zhi_index(&self) -> i64 {
        if self.sect == 2 { self.lunar.day_zhi_index_exact2() } else { self.lunar.day_zhi_index_exact() }
    }

    fn di_shi(&self, zhi_index: i64) -> &'static str {
        let mut index = chang_sheng_offset(self.day_gan());
        if self.day_gan_index() % 2 == 0 {
            index += zhi_index;
        } else {
            index -= zhi_index;
        }
        if index >= 12 {
            index -= 12;
        }
        if index < 0 {
            index += 12;
        }
        CHANG_SHENG[index as usize]
    }

    fn terrain(&self, zhi_index: i64) -> Terrain {
        Terrain::from_name(self.di_shi(zhi_index)).unwrap_or_else(|| Terrain::from_index(0))
    }

    // ---- 年柱 ----
    pub fn year(&self) -> String {
        self.lunar.year_in_gan_zhi_exact()
    }
    pub fn year_pillar(&self) -> SixtyCycleYear {
        SixtyCycleYear::new(SixtyCycle::from_name(&self.year()).unwrap_or_else(|| SixtyCycle::from_index(0)))
    }
    pub fn year_gan(&self) -> &'static str {
        self.lunar.year_gan_exact()
    }
    pub fn year_zhi(&self) -> &'static str {
        self.lunar.year_zhi_exact()
    }
    pub fn year_hide_gan(&self) -> &'static [&'static str] {
        lunar_util::zhi_hide_gan(self.year_zhi())
    }
    pub fn year_wu_xing(&self) -> String {
        format!("{}{}", lunar_util::wu_xing_gan(self.year_gan()), lunar_util::wu_xing_zhi(self.year_zhi()))
    }
    pub fn year_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.year())
    }
    pub fn year_shi_shen_gan(&self) -> &'static str {
        lunar_util::shi_shen(&format!("{}{}", self.day_gan(), self.year_gan()))
    }
    pub fn year_shi_shen_zhi(&self) -> Vec<&'static str> {
        self.shi_shen_zhi(self.year_zhi())
    }
    pub fn year_di_shi(&self) -> &'static str {
        self.di_shi(self.lunar.year_zhi_index_exact())
    }
    pub fn year_terrain(&self) -> Terrain {
        self.terrain(self.lunar.year_zhi_index_exact())
    }

    // ---- 月柱 ----
    pub fn month(&self) -> String {
        self.lunar.month_in_gan_zhi_exact()
    }
    pub fn month_pillar(&self) -> SixtyCycleMonth {
        SixtyCycleMonth::new(SixtyCycle::from_name(&self.month()).unwrap_or_else(|| SixtyCycle::from_index(0)))
    }
    pub fn month_gan(&self) -> &'static str {
        self.lunar.month_gan_exact()
    }
    pub fn month_zhi(&self) -> &'static str {
        self.lunar.month_zhi_exact()
    }
    pub fn month_hide_gan(&self) -> &'static [&'static str] {
        lunar_util::zhi_hide_gan(self.month_zhi())
    }
    pub fn month_wu_xing(&self) -> String {
        format!("{}{}", lunar_util::wu_xing_gan(self.month_gan()), lunar_util::wu_xing_zhi(self.month_zhi()))
    }
    pub fn month_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.month())
    }
    pub fn month_shi_shen_gan(&self) -> &'static str {
        lunar_util::shi_shen(&format!("{}{}", self.day_gan(), self.month_gan()))
    }
    pub fn month_shi_shen_zhi(&self) -> Vec<&'static str> {
        self.shi_shen_zhi(self.month_zhi())
    }
    pub fn month_di_shi(&self) -> &'static str {
        self.di_shi(self.lunar.month_zhi_index_exact())
    }
    pub fn month_terrain(&self) -> Terrain {
        self.terrain(self.lunar.month_zhi_index_exact())
    }

    // ---- 日柱 ----
    pub fn day(&self) -> String {
        if self.sect == 2 { self.lunar.day_in_gan_zhi_exact2() } else { self.lunar.day_in_gan_zhi_exact() }
    }
    pub fn day_pillar(&self) -> SixtyCycleDay {
        SixtyCycleDay::new(SixtyCycle::from_name(&self.day()).unwrap_or_else(|| SixtyCycle::from_index(0)))
    }
    pub fn day_gan(&self) -> &'static str {
        if self.sect == 2 { self.lunar.day_gan_exact2() } else { self.lunar.day_gan_exact() }
    }
    pub fn day_zhi(&self) -> &'static str {
        if self.sect == 2 { self.lunar.day_zhi_exact2() } else { self.lunar.day_zhi_exact() }
    }
    pub fn day_hide_gan(&self) -> &'static [&'static str] {
        lunar_util::zhi_hide_gan(self.day_zhi())
    }
    pub fn day_wu_xing(&self) -> String {
        format!("{}{}", lunar_util::wu_xing_gan(self.day_gan()), lunar_util::wu_xing_zhi(self.day_zhi()))
    }
    pub fn day_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.day())
    }
    pub const fn day_shi_shen_gan(&self) -> &'static str {
        "日主"
    }
    pub fn day_shi_shen_zhi(&self) -> Vec<&'static str> {
        self.shi_shen_zhi(self.day_zhi())
    }
    pub fn day_di_shi(&self) -> &'static str {
        self.di_shi(self.lunar.day_zhi_index_exact())
    }
    pub fn day_terrain(&self) -> Terrain {
        self.terrain(self.lunar.day_zhi_index_exact())
    }

    // ---- 时柱 ----
    pub fn time(&self) -> String {
        self.lunar.time_in_gan_zhi()
    }
    pub fn time_pillar(&self) -> SixtyCycleHour {
        SixtyCycleHour::new(SixtyCycle::from_name(&self.time()).unwrap_or_else(|| SixtyCycle::from_index(0)))
    }
    pub fn time_gan(&self) -> &'static str {
        self.lunar.time_gan()
    }
    pub fn time_zhi(&self) -> &'static str {
        self.lunar.time_zhi()
    }
    pub fn time_hide_gan(&self) -> &'static [&'static str] {
        lunar_util::zhi_hide_gan(self.time_zhi())
    }
    pub fn time_wu_xing(&self) -> String {
        format!("{}{}", lunar_util::wu_xing_gan(self.time_gan()), lunar_util::wu_xing_zhi(self.time_zhi()))
    }
    pub fn time_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.time())
    }
    pub fn time_shi_shen_gan(&self) -> &'static str {
        lunar_util::shi_shen(&format!("{}{}", self.day_gan(), self.time_gan()))
    }
    pub fn time_shi_shen_zhi(&self) -> Vec<&'static str> {
        self.shi_shen_zhi(self.time_zhi())
    }
    pub fn time_di_shi(&self) -> &'static str {
        self.di_shi(self.lunar.time_zhi_index())
    }
    pub fn time_terrain(&self) -> Terrain {
        self.terrain(self.lunar.time_zhi_index())
    }

    pub fn three_pillars(&self) -> ThreePillars {
        ThreePillars::new(self.year_pillar(), self.month_pillar(), self.day_pillar())
    }

    // ---- 胎元 / 胎息 / 命宫 / 身宫 ----
    pub fn tai_yuan(&self) -> String {
        let mut gan_index = self.lunar.month_gan_index_exact() + 1;
        if gan_index >= 10 {
            gan_index -= 10;
        }
        let mut zhi_index = self.lunar.month_zhi_index_exact() + 3;
        if zhi_index >= 12 {
            zhi_index -= 12;
        }
        format!(
            "{}{}",
            lunar_util::tables::GAN[(gan_index + 1) as usize],
            lunar_util::tables::ZHI[(zhi_index + 1) as usize]
        )
    }
    pub fn tai_yuan_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.tai_yuan())
    }

    pub fn tai_xi(&self) -> String {
        let (mut gan_index, mut zhi_index) = (self.lunar.day_gan_index_exact(), self.lunar.day_zhi_index_exact());
        if self.sect == 2 {
            gan_index = self.lunar.day_gan_index_exact2();
            zhi_index = self.lunar.day_zhi_index_exact2();
        }
        format!(
            "{}{}",
            lunar_util::tables::HE_GAN_5[gan_index as usize],
            lunar_util::tables::HE_ZHI_6[zhi_index as usize]
        )
    }
    pub fn tai_xi_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.tai_xi())
    }

    fn month_time_zhi_indices(&self) -> (i64, i64) {
        let month_zhi = self.month_zhi();
        let time_zhi = self.time_zhi();
        let mut month_zhi_index = 0_i64;
        for (i, v) in MONTH_ZHI.iter().enumerate() {
            if *v == month_zhi {
                month_zhi_index = i as i64;
                break;
            }
        }
        let mut time_zhi_index = 0_i64;
        for (i, v) in MONTH_ZHI.iter().enumerate() {
            if *v == time_zhi {
                time_zhi_index = i as i64;
                break;
            }
        }
        (month_zhi_index, time_zhi_index)
    }

    pub fn ming_gong(&self) -> String {
        let (month_zhi_index, time_zhi_index) = self.month_time_zhi_indices();
        let offset = if month_zhi_index + time_zhi_index >= 14 {
            26 - (month_zhi_index + time_zhi_index)
        } else {
            14 - (month_zhi_index + time_zhi_index)
        };
        let mut gan_index = (self.lunar.year_gan_index_exact() + 1) * 2 + offset;
        while gan_index > 10 {
            gan_index -= 10;
        }
        format!("{}{}", lunar_util::tables::GAN[gan_index as usize], MONTH_ZHI[offset as usize])
    }
    pub fn ming_gong_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.ming_gong())
    }

    pub fn shen_gong(&self) -> String {
        let (month_zhi_index, _) = self.month_time_zhi_indices();
        let time_zhi = self.time_zhi();
        let mut time_zhi_index = 0_i64;
        for (i, v) in lunar_util::tables::ZHI.iter().enumerate() {
            if *v == time_zhi {
                time_zhi_index = i as i64;
                break;
            }
        }
        let mut offset = month_zhi_index + time_zhi_index;
        if offset > 12 {
            offset -= 12;
        }
        let mut gan_index = (self.lunar.year_gan_index_exact() + 1) * 2 + offset;
        while gan_index > 10 {
            gan_index -= 10;
        }
        format!("{}{}", lunar_util::tables::GAN[gan_index as usize], MONTH_ZHI[offset as usize])
    }
    pub fn shen_gong_na_yin(&self) -> &'static str {
        lunar_util::nayin(&self.shen_gong())
    }

    // ---- 运 ----
    pub fn yun(&self, gender: Gender) -> Yun<'_> {
        self.yun_by_sect(gender, 1)
    }
    pub fn yun_by_sect(&self, gender: Gender, sect: u8) -> Yun<'_> {
        Yun::new(self.lunar, gender, sect)
    }
    pub fn child_limit(&self, gender: Gender) -> ChildLimit<'_> {
        self.child_limit_with_provider(gender, &DefaultChildLimitProvider::new())
    }
    pub fn child_limit_with_provider<P>(&self, gender: Gender, provider: &P) -> ChildLimit<'_>
    where
        P: ChildLimitProvider + ?Sized,
    {
        provider.child_limit(self.lunar, gender)
    }

    // ---- 旬 / 空亡 ----
    pub fn year_xun(&self) -> &'static str {
        self.lunar.year_xun_exact()
    }
    pub fn year_xun_kong(&self) -> &'static str {
        self.lunar.year_xun_kong_exact()
    }
    pub fn month_xun(&self) -> &'static str {
        self.lunar.month_xun_exact()
    }
    pub fn month_xun_kong(&self) -> &'static str {
        self.lunar.month_xun_kong_exact()
    }
    pub fn day_xun(&self) -> &'static str {
        if self.sect == 2 { self.lunar.day_xun_exact2() } else { self.lunar.day_xun_exact() }
    }
    pub fn day_xun_kong(&self) -> &'static str {
        if self.sect == 2 { self.lunar.day_xun_kong_exact2() } else { self.lunar.day_xun_kong_exact() }
    }
    pub fn time_xun(&self) -> &'static str {
        self.lunar.time_xun()
    }
    pub fn time_xun_kong(&self) -> &'static str {
        self.lunar.time_xun_kong()
    }
}
