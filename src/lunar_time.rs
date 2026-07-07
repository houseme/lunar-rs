//! 时辰（两小时为一时辰）。对应 lunar-go `calendar/LunarTime.go`。

use std::marker::PhantomData;

use crate::culture::{
    ChongSha, Direction, EarthBranch, HeavenStem, MinorRen, Nayin, SixtyCycle, SixtyCycleDay, SixtyCycleHour,
    SixtyCycleMonth, SixtyCycleYear, Taboo, TabooKind, TianShen, TwelveStar, Xun, Zodiac,
};
use crate::eight_char::EightChar;
use crate::lunar::Lunar;
use crate::lunar_util;
use crate::nine_star::NineStar;
use crate::solar::Solar;

/// 时辰。内部持有一个 [`Lunar`] 快照，兼容对外的 lifetime 形状。
#[derive(Clone)]
pub struct LunarTime<'a> {
    gan_index: i64,
    zhi_index: i64,
    lunar: Lunar,
    marker: PhantomData<&'a Lunar>,
}

impl<'a> LunarTime<'a> {
    pub(crate) fn from_lunar(lunar: &'a Lunar) -> Self {
        let zhi_index = lunar_util::time_zhi_index_from_hour(lunar.hour());
        let gan_index = (lunar.day_gan_index_exact() % 5 * 2 + zhi_index) % 10;
        Self { gan_index, zhi_index, lunar: lunar.clone(), marker: PhantomData }
    }

    pub fn from_ymd_hms(
        year: i32,
        month: i32,
        day: i32,
        hour: i32,
        minute: i32,
        second: i32,
    ) -> Result<LunarTime<'static>, crate::LunarError> {
        let lunar = Lunar::from_ymd_hms(year, month, day, hour, minute, second)?;
        let zhi_index = lunar_util::time_zhi_index_from_hour(lunar.hour());
        let gan_index = (lunar.day_gan_index_exact() % 5 * 2 + zhi_index) % 10;
        Ok(LunarTime { gan_index, zhi_index, lunar, marker: PhantomData })
    }

    pub const fn gan_index(&self) -> i64 {
        self.gan_index
    }
    pub const fn zhi_index(&self) -> i64 {
        self.zhi_index
    }

    pub fn get_lunar_day(&self) -> Lunar {
        self.lunar.clone()
    }

    pub const fn get_year(&self) -> i32 {
        self.lunar.year()
    }

    pub const fn get_month(&self) -> i32 {
        self.lunar.month()
    }

    pub const fn get_day(&self) -> i32 {
        self.lunar.day()
    }

    pub const fn get_hour(&self) -> i32 {
        self.lunar.hour()
    }

    pub const fn get_minute(&self) -> i32 {
        self.lunar.minute()
    }

    pub const fn get_second(&self) -> i32 {
        self.lunar.second()
    }

    pub const fn get_index_in_day(&self) -> usize {
        ((self.get_hour() + 1) / 2) as usize
    }

    pub fn gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.gan_index + 1) as usize]
    }
    pub fn heaven_stem(&self) -> HeavenStem {
        HeavenStem::from_index(self.gan_index as usize)
    }
    pub fn zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.zhi_index + 1) as usize]
    }
    pub fn earth_branch(&self) -> EarthBranch {
        EarthBranch::from_index(self.zhi_index as usize)
    }
    pub fn gan_zhi(&self) -> String {
        format!("{}{}", self.gan(), self.zhi())
    }
    pub fn sixty_cycle(&self) -> SixtyCycle {
        SixtyCycle::from_name(&self.gan_zhi()).expect("time ganzhi must map to sixty-cycle")
    }
    pub fn get_sixty_cycle(&self) -> SixtyCycle {
        self.sixty_cycle()
    }
    pub fn sixty_cycle_hour(&self) -> SixtyCycleHour {
        SixtyCycleHour::new(self.sixty_cycle())
    }
    pub fn get_sixty_cycle_hour(&self) -> SixtyCycleHour {
        self.sixty_cycle_hour()
    }
    pub fn get_year_sixty_cycle(&self) -> SixtyCycle {
        self.lunar.year_sixty_cycle()
    }
    pub fn get_month_sixty_cycle(&self) -> SixtyCycle {
        self.lunar.month_sixty_cycle()
    }
    pub fn get_day_sixty_cycle(&self) -> SixtyCycle {
        self.lunar.day_sixty_cycle()
    }
    pub fn get_sixty_cycle_year(&self) -> SixtyCycleYear {
        self.lunar.sixty_cycle_year()
    }
    pub fn get_sixty_cycle_month(&self) -> SixtyCycleMonth {
        self.lunar.sixty_cycle_month()
    }
    pub fn get_sixty_cycle_day(&self) -> SixtyCycleDay {
        self.lunar.sixty_cycle_day()
    }
    pub fn minor_ren(&self) -> MinorRen {
        let index = self.lunar.day_minor_ren().index() as i64 + self.zhi_index;
        MinorRen::from_index(index.rem_euclid(6) as usize)
    }
    pub fn get_minor_ren(&self) -> MinorRen {
        self.minor_ren()
    }
    pub fn sheng_xiao(&self) -> &'static str {
        lunar_util::tables::SHENG_XIAO[(self.zhi_index + 1) as usize]
    }

    pub fn position_xi(&self) -> &'static str {
        lunar_util::tables::POSITION_XI[(self.gan_index + 1) as usize]
    }
    pub fn position_xi_direction(&self) -> Direction {
        Direction::new(self.position_xi())
    }
    pub fn position_xi_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_xi())
    }
    pub fn position_yang_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YANG_GUI[(self.gan_index + 1) as usize]
    }
    pub fn position_yang_gui_direction(&self) -> Direction {
        Direction::new(self.position_yang_gui())
    }
    pub fn position_yang_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_yang_gui())
    }
    pub fn position_yin_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YIN_GUI[(self.gan_index + 1) as usize]
    }
    pub fn position_yin_gui_direction(&self) -> Direction {
        Direction::new(self.position_yin_gui())
    }
    pub fn position_yin_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_yin_gui())
    }
    pub fn position_fu(&self) -> &'static str {
        self.position_fu_by_sect(2)
    }
    pub fn position_fu_direction(&self) -> Direction {
        Direction::new(self.position_fu())
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
    pub fn position_cai_direction(&self) -> Direction {
        Direction::new(self.position_cai())
    }
    pub fn position_cai_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_cai())
    }

    pub fn nayin(&self) -> &'static str {
        lunar_util::nayin(&self.gan_zhi())
    }
    pub fn nayin_info(&self) -> Nayin {
        Nayin::new(self.nayin())
    }

    pub fn tian_shen(&self) -> &'static str {
        let off = lunar_util::ZHI_TIAN_SHEN_OFFSET
            [lunar_util::find(self.lunar.day_zhi_exact(), lunar_util::tables::ZHI, 0) as usize];
        lunar_util::tables::TIAN_SHEN[((self.zhi_index + off) % 12 + 1) as usize]
    }
    pub fn tian_shen_info(&self) -> TianShen {
        TianShen::new(self.tian_shen())
    }
    pub fn get_twelve_star(&self) -> TwelveStar {
        TwelveStar::from_name(self.tian_shen()).unwrap_or_else(|| TwelveStar::from_index(0))
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
    pub fn chong_sha(&self) -> ChongSha {
        ChongSha::new(self.chong_gan(), self.chong(), Zodiac::new(self.chong_sheng_xiao()), Direction::new(self.sha()))
    }

    pub fn yi(&self) -> Vec<&'static str> {
        lunar_util::get_time_yi(&self.lunar.day_in_gan_zhi_exact(), &self.gan_zhi())
    }
    pub fn ji(&self) -> Vec<&'static str> {
        lunar_util::get_time_ji(&self.lunar.day_in_gan_zhi_exact(), &self.gan_zhi())
    }
    pub fn get_recommends(&self) -> Vec<Taboo> {
        self.yi().into_iter().map(|name| Taboo::new(name, TabooKind::Recommend)).collect()
    }
    pub fn get_avoids(&self) -> Vec<Taboo> {
        self.ji().into_iter().map(|name| Taboo::new(name, TabooKind::Avoid)).collect()
    }

    pub fn nine_star(&self) -> NineStar {
        let solar_ymd = self.lunar.solar().to_ymd();
        let jq = self.lunar.jie_qi_table();
        let dong_zhi = jq.get("冬至").copied().unwrap_or_else(|| self.lunar.solar());
        let xia_zhi = jq.get("夏至").copied().unwrap_or_else(|| self.lunar.solar());
        let asc = solar_ymd >= dong_zhi.to_ymd() && solar_ymd < xia_zhi.to_ymd();
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
    pub fn get_nine_star(&self) -> NineStar {
        self.nine_star()
    }

    pub fn xun(&self) -> &'static str {
        lunar_util::get_xun(&self.gan_zhi())
    }
    pub fn xun_kong(&self) -> &'static str {
        lunar_util::get_xun_kong(&self.gan_zhi())
    }
    pub fn xun_info(&self) -> Xun {
        Xun::new(self.xun(), self.xun_kong())
    }

    pub fn get_solar_time(&self) -> Solar {
        self.lunar.solar()
    }

    pub fn get_eight_char(&self) -> EightChar<'_> {
        self.lunar.eight_char()
    }

    pub fn is_before(&self, target: &Self) -> bool {
        self.get_solar_time().is_before(&target.get_solar_time())
    }

    pub fn is_after(&self, target: &Self) -> bool {
        self.get_solar_time().is_after(&target.get_solar_time())
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
