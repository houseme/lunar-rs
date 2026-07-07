//! 农历年：寿星天文历 master 计算（节气、合朔、闰月、月序）。
//!
//! 对应 lunar-go `calendar/LunarYear.go`。每年的天文 pass 仅执行一次，结果缓存复用。

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

use crate::culture::{
    Direction, EarthBranch, HeavenStem, KitchenGodSteed, Nayin, SixtyCycle, TaiSuiPosition, Xun, YearFortune,
    YearFortuneKind, YuanCycle, YunCycle,
};
use crate::lunar_month::LunarMonth;
use crate::lunar_util;
use crate::multi_calendar::CalendarSpan;
use crate::nine_star::NineStar;
use crate::shou_xing;
use crate::solar::Solar;

/// 二十四节气（冬至起算）。
pub const JIE_QI: &[&str; 24] = &[
    "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至", "小暑",
    "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪",
];

/// 31 项节气查找表（含跨年边界的拼音 token 副本）。**顺序承载逻辑**。
pub const JIE_QI_IN_USE: &[&str; 31] = &[
    "DA_XUE", "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至",
    "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪", "DONG_ZHI", "XIAO_HAN",
    "DA_HAN", "LI_CHUN", "YU_SHUI", "JING_ZHE",
];

const YUAN: &[&str; 3] = &["下", "上", "中"];
const YUN: &[&str; 9] = &["七", "八", "九", "一", "二", "三", "四", "五", "六"];
const YMC: &[i64; 12] = &[11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 历史闰月兜底表（闰 11 月 / 闰 12 月的年份）。
const LEAP_11: &[i32] = &[
    75, 94, 170, 265, 322, 398, 469, 553, 583, 610, 678, 735, 754, 773, 849, 887, 936, 1050, 1069, 1126, 1145, 1164,
    1183, 1259, 1278, 1308, 1373, 1403, 1441, 1460, 1498, 1555, 1593, 1612, 1631, 1642, 2033, 2128, 2147, 2242, 2614,
    2728, 2910, 3062, 3244, 3339, 3616, 3711, 3730, 3825, 4007, 4159, 4197, 4322, 4341, 4379, 4417, 4531, 4599, 4694,
    4713, 4789, 4808, 4971, 5085, 5104, 5161, 5180, 5199, 5294, 5305, 5476, 5677, 5696, 5772, 5791, 5848, 5886, 6049,
    6068, 6144, 6163, 6258, 6402, 6440, 6497, 6516, 6630, 6641, 6660, 6679, 6736, 6774, 6850, 6869, 6899, 6918, 6994,
    7013, 7032, 7051, 7070, 7089, 7108, 7127, 7146, 7222, 7271, 7290, 7309, 7366, 7385, 7404, 7442, 7461, 7480, 7491,
    7499, 7594, 7624, 7643, 7662, 7681, 7719, 7738, 7814, 7863, 7882, 7901, 7939, 7958, 7977, 7996, 8034, 8053, 8072,
    8091, 8121, 8159, 8186, 8216, 8235, 8254, 8273, 8311, 8330, 8341, 8349, 8368, 8444, 8463, 8474, 8493, 8531, 8569,
    8588, 8626, 8664, 8683, 8694, 8702, 8713, 8721, 8751, 8789, 8808, 8816, 8827, 8846, 8884, 8903, 8922, 8941, 8971,
    9036, 9066, 9085, 9104, 9123, 9142, 9161, 9180, 9199, 9218, 9256, 9294, 9313, 9324, 9343, 9362, 9381, 9419, 9438,
    9476, 9514, 9533, 9544, 9552, 9563, 9571, 9582, 9601, 9639, 9658, 9666, 9677, 9696, 9734, 9753, 9772, 9791, 9802,
    9821, 9886, 9897, 9916, 9935, 9954, 9973, 9992,
];
const LEAP_12: &[i32] = &[
    37, 56, 113, 132, 151, 189, 208, 227, 246, 284, 303, 341, 360, 379, 417, 436, 458, 477, 496, 515, 534, 572, 591,
    629, 648, 667, 697, 716, 792, 811, 830, 868, 906, 925, 944, 963, 982, 1001, 1020, 1039, 1058, 1088, 1153, 1202,
    1221, 1240, 1297, 1335, 1392, 1411, 1422, 1430, 1517, 1525, 1536, 1574, 3358, 3472, 3806, 3988, 4751, 4941, 5066,
    5123, 5275, 5343, 5438, 5457, 5495, 5533, 5552, 5715, 5810, 5829, 5905, 5924, 6421, 6535, 6793, 6812, 6888, 6907,
    7002, 7184, 7260, 7279, 7374, 7556, 7746, 7757, 7776, 7833, 7852, 7871, 7966, 8015, 8110, 8129, 8148, 8224, 8243,
    8338, 8406, 8425, 8482, 8501, 8520, 8558, 8596, 8607, 8615, 8645, 8740, 8778, 8835, 8865, 8930, 8960, 8979, 8998,
    9017, 9055, 9074, 9093, 9112, 9150, 9188, 9237, 9275, 9332, 9351, 9370, 9408, 9427, 9446, 9457, 9465, 9495, 9560,
    9590, 9628, 9647, 9685, 9715, 9742, 9780, 9810, 9818, 9829, 9848, 9867, 9905, 9924, 9943, 9962, 10000,
];

static CACHE: LazyLock<RwLock<HashMap<i32, Arc<LunarYear>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));
const YMC_SHIFT_A_START: f64 = 1_724_360.0;
const YMC_SHIFT_A_END: f64 = 1_729_794.0;
const YMC_SHIFT_B_START: f64 = 1_807_724.0;
const YMC_SHIFT_B_END: f64 = 1_808_699.0;

fn is_ymc_boundary(dm: f64) -> bool {
    (dm - YMC_SHIFT_A_END).abs() < f64::EPSILON || (dm - YMC_SHIFT_B_END).abs() < f64::EPSILON
}

/// 农历年。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LunarYear {
    pub(crate) year: i32,
    pub(crate) gan_index: i64,
    pub(crate) zhi_index: i64,
    months: Vec<LunarMonth>,
    jie_qi_julian_days: [f64; 31],
}

impl LunarYear {
    /// 获取指定农历年（命中缓存复用）。
    pub fn from_year(year: i32) -> Arc<Self> {
        {
            let cache = CACHE.read().unwrap();
            if let Some(y) = cache.get(&year) {
                return Arc::clone(y);
            }
        }
        let mut y =
            Self { year, gan_index: 0, zhi_index: 0, months: Vec::with_capacity(15), jie_qi_julian_days: [0.0; 31] };
        let offset = year - 4;
        let mut gan_index = i64::from(offset.rem_euclid(10));
        let mut zhi_index = i64::from(offset.rem_euclid(12));
        if gan_index < 0 {
            gan_index += 10;
        }
        if zhi_index < 0 {
            zhi_index += 12;
        }
        y.gan_index = gan_index;
        y.zhi_index = zhi_index;
        y.compute();

        let arc = Arc::new(y);
        let mut cache = CACHE.write().unwrap();
        cache.insert(year, Arc::clone(&arc));
        arc
    }

    /// 寿星天文历 master 计算。
    fn compute(&mut self) {
        let mut jq = [0.0_f64; 27];
        let mut hs = [0.0_f64; 16];
        let mut day_counts = [0_i64; 15];
        let mut months_idx = [0_i64; 15];

        let current_year = self.year;
        let jd = (f64::from(current_year - 2000) * 365.2422 + 180.0).floor();
        let mut w = ((jd - 355.0 + 183.0) / 365.2422).floor().mul_add(365.2422, 355.0);
        if shou_xing::calc_qi(w) > jd {
            w -= 365.2422;
        }
        for (i, item) in jq.iter_mut().enumerate().take(26) {
            *item = shou_xing::calc_qi(15.2184f64.mul_add(i as f64, w));
        }
        let j = JIE_QI_IN_USE.len();
        for i in 0..j {
            let d = if i == 0 {
                shou_xing::qi_accurate_2(jq[0] - 15.2184)
            } else if i <= 26 {
                shou_xing::qi_accurate_2(jq[i - 1])
            } else {
                shou_xing::qi_accurate_2(15.2184f64.mul_add(i as f64 - 26.0, jq[25]))
            };
            self.jie_qi_julian_days[i] = d + shou_xing::J2000;
        }

        let mut w2 = shou_xing::calc_shuo(jq[0]);
        if w2 > jq[0] {
            w2 -= 29.53;
        }
        for (i, item) in hs.iter_mut().enumerate() {
            *item = shou_xing::calc_shuo(29.5306f64.mul_add(i as f64, w2));
        }
        for i in 0..15 {
            day_counts[i] = (hs[i + 1] - hs[i]) as i64;
            months_idx[i] = i as i64;
        }

        let prev_year = current_year - 1;
        let mut leap_index: i64 = 16;
        if LEAP_11.contains(&current_year) {
            leap_index = 13;
        } else if LEAP_12.contains(&current_year) {
            leap_index = 14;
        } else if hs[13] <= jq[24] {
            let mut i = 1_i64;
            loop {
                if hs[(i + 1) as usize] <= jq[(2 * i) as usize] {
                    break;
                }
                if i >= 13 {
                    break;
                }
                i += 1;
            }
            leap_index = i;
        }
        for i in leap_index..15 {
            months_idx[i as usize] -= 1;
        }

        let mut fm: i64 = -1;
        let mut index: i64 = -1;
        let mut y = prev_year;
        for i in 0..15 {
            let dm = hs[i] + shou_xing::J2000;
            let v2 = months_idx[i];
            let mut mc = YMC[(v2 % 12) as usize];
            if (YMC_SHIFT_A_START..YMC_SHIFT_A_END).contains(&dm) || (YMC_SHIFT_B_START..YMC_SHIFT_B_END).contains(&dm)
            {
                mc = YMC[((v2 + 1) % 12) as usize];
            } else if is_ymc_boundary(dm) {
                mc = 12;
            }
            if fm == -1 {
                fm = mc;
                index = mc;
            }
            if mc < fm {
                y += 1;
                index = 1;
            }
            fm = mc;
            let mut month_code = mc;
            if i as i64 == leap_index {
                month_code = -month_code;
            } else if is_ymc_boundary(dm) {
                month_code = -11;
            }
            self.months.push(LunarMonth::new(
                y,
                month_code as i32,
                day_counts[i] as i32,
                hs[i] + shou_xing::J2000,
                index as i32,
            ));
            index += 1;
        }
    }

    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }
    pub const fn get_year(&self) -> i32 {
        self.year()
    }
    #[inline]
    pub const fn gan_index(&self) -> i64 {
        self.gan_index
    }
    #[inline]
    pub const fn zhi_index(&self) -> i64 {
        self.zhi_index
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
        SixtyCycle::from_name(&self.gan_zhi()).expect("year ganzhi must map to sixty-cycle")
    }
    pub fn get_sixty_cycle(&self) -> SixtyCycle {
        self.sixty_cycle()
    }
    pub fn nayin(&self) -> &'static str {
        lunar_util::nayin(&self.gan_zhi())
    }
    pub fn nayin_info(&self) -> Nayin {
        Nayin::new(self.nayin())
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

    /// 全部 15 个月（含跨年边界）。
    pub fn months(&self) -> Vec<LunarMonth> {
        self.months.clone()
    }
    pub fn get_months(&self) -> Vec<LunarMonth> {
        self.months_in_year().collect()
    }
    /// 当年所属月份。
    pub fn months_in_year(&self) -> impl Iterator<Item = LunarMonth> + '_ {
        self.months.iter().copied().filter(move |m| m.year == self.year)
    }
    /// 当年总天数。
    pub fn day_count(&self) -> i32 {
        self.months.iter().filter(|m| m.year == self.year).map(|m| m.day_count).sum()
    }
    pub fn get_day_count(&self) -> i32 {
        self.day_count()
    }
    pub fn get_month_count(&self) -> usize {
        self.get_months().len()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.months_in_year().next().unwrap().first_solar_day()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.months_in_year().last().unwrap().last_solar_day()
    }
    /// 31 个节气儒略日。
    pub const fn jie_qi_julian_days(&self) -> &[f64; 31] {
        &self.jie_qi_julian_days
    }
    /// 查找当年指定月（返回 `None` 表示该年无此月）。
    pub fn get_month(&self, lunar_month: i32) -> Option<LunarMonth> {
        self.months.iter().copied().find(|m| m.year == self.year && m.month == lunar_month)
    }
    /// 当年闰月（0 表示无闰）。
    pub fn leap_month(&self) -> i32 {
        for m in &self.months {
            if m.year == self.year && m.is_leap() {
                return m.month.abs();
            }
        }
        0
    }
    pub fn get_leap_month(&self) -> i32 {
        self.leap_month()
    }

    /// 元（上 / 中 / 下元）。
    pub fn yuan(&self) -> String {
        let i = (((self.year + 2696) / 60) % 3) as usize;
        format!("{}元", YUAN[i])
    }
    pub fn yuan_cycle(&self) -> YuanCycle {
        YuanCycle::new(self.yuan())
    }
    /// 运。
    pub fn yun(&self) -> String {
        let i = (((self.year + 2696) / 20) % 9) as usize;
        format!("{}运", YUN[i])
    }
    pub fn yun_cycle(&self) -> YunCycle {
        YunCycle::new(self.yun())
    }
    pub fn get_twenty(&self) -> crate::Twenty {
        let index = (self.year - 1864).div_euclid(20).rem_euclid(9) as usize;
        crate::Twenty::from_index(index)
    }
    pub fn get_jupiter_direction(&self) -> Direction {
        let branch_index = self.get_sixty_cycle().earth_branch().index();
        Direction::from_index([0, 7, 7, 2, 3, 3, 8, 1, 1, 6, 0, 0][branch_index])
    }
    /// 年九星。
    pub fn nine_star(&self) -> NineStar {
        let index = lunar_util::get_jia_zi_index(&self.gan_zhi()) + 1;
        let yuan = i64::from(((self.year + 2696) / 60) % 3);
        let mut offset = (62 + yuan * 3 - index) % 9;
        if offset == 0 {
            offset = 9;
        }
        NineStar::from_index(offset - 1)
    }
    pub fn get_nine_star(&self) -> NineStar {
        self.nine_star()
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
    pub fn position_fu_desc_by_sect(&self, sect: u8) -> &'static str {
        lunar_util::position_desc(self.position_fu_by_sect(sect))
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
    pub fn position_tai_sui(&self) -> &'static str {
        lunar_util::tables::POSITION_TAI_SUI_YEAR[self.zhi_index as usize]
    }
    pub fn tai_sui_position(&self) -> TaiSuiPosition {
        TaiSuiPosition::new(Direction::new(self.position_tai_sui()), self.position_tai_sui_desc())
    }
    pub fn position_tai_sui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_tai_sui())
    }

    pub fn next(&self, n: i32) -> Arc<Self> {
        Self::from_year(self.year + n)
    }

    // ---- 杂占（几鼠偷粮 / 几牛耕田 等）----
    fn first_day_gan_index(&self) -> i64 {
        let m1 = self.get_month(1).unwrap();
        Solar::from_julian_day(m1.first_julian_day).lunar().day_gan_index()
    }
    fn first_day_zhi_index(&self) -> i64 {
        let m1 = self.get_month(1).unwrap();
        Solar::from_julian_day(m1.first_julian_day).lunar().day_zhi_index()
    }
    fn zao_by_gan(&self, index: i64, name: &str) -> String {
        let mut offset = index - self.first_day_gan_index();
        if offset < 0 {
            offset += 10;
        }
        name.replace("几", lunar_util::tables::NUMBER[(offset + 1) as usize])
    }
    fn zao_by_zhi(&self, index: i64, name: &str) -> String {
        let mut offset = index - self.first_day_zhi_index();
        if offset < 0 {
            offset += 12;
        }
        name.replace("几", lunar_util::tables::NUMBER[(offset + 1) as usize])
    }
    pub fn tou_liang(&self) -> String {
        self.zao_by_zhi(0, "几鼠偷粮")
    }
    pub fn tou_liang_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::TouLiang, self.tou_liang())
    }
    pub fn cao_zi(&self) -> String {
        self.zao_by_zhi(0, "草子几分")
    }
    pub fn cao_zi_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::CaoZi, self.cao_zi())
    }
    pub fn geng_tian(&self) -> String {
        self.zao_by_zhi(1, "几牛耕田")
    }
    pub fn geng_tian_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::GengTian, self.geng_tian())
    }
    pub fn hua_shou(&self) -> String {
        self.zao_by_zhi(3, "花收几分")
    }
    pub fn hua_shou_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::HuaShou, self.hua_shou())
    }
    pub fn zhi_shui(&self) -> String {
        self.zao_by_zhi(4, "几龙治水")
    }
    pub fn zhi_shui_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::ZhiShui, self.zhi_shui())
    }
    pub fn tuo_gu(&self) -> String {
        self.zao_by_zhi(6, "几马驮谷")
    }
    pub fn tuo_gu_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::TuoGu, self.tuo_gu())
    }
    pub fn qiang_mi(&self) -> String {
        self.zao_by_zhi(9, "几鸡抢米")
    }
    pub fn qiang_mi_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::QiangMi, self.qiang_mi())
    }
    pub fn kan_can(&self) -> String {
        self.zao_by_zhi(9, "几姑看蚕")
    }
    pub fn kan_can_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::KanCan, self.kan_can())
    }
    pub fn gong_zhu(&self) -> String {
        self.zao_by_zhi(11, "几屠共猪")
    }
    pub fn gong_zhu_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::GongZhu, self.gong_zhu())
    }
    pub fn jia_tian(&self) -> String {
        self.zao_by_gan(0, "甲田几分")
    }
    pub fn jia_tian_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::JiaTian, self.jia_tian())
    }
    pub fn fen_bing(&self) -> String {
        self.zao_by_gan(2, "几人分饼")
    }
    pub fn fen_bing_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::FenBing, self.fen_bing())
    }
    pub fn de_jin(&self) -> String {
        self.zao_by_gan(7, "几日得金")
    }
    pub fn de_jin_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::DeJin, self.de_jin())
    }
    pub fn ren_bing(&self) -> String {
        self.kitchen_god_steed().people_cakes()
    }
    pub fn ren_bing_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::RenBing, self.ren_bing())
    }
    pub fn ren_chu(&self) -> String {
        self.kitchen_god_steed().people_hoes()
    }
    pub fn ren_chu_info(&self) -> YearFortune {
        YearFortune::new(YearFortuneKind::RenChu, self.ren_chu())
    }

    pub fn kitchen_god_steed(&self) -> KitchenGodSteed {
        KitchenGodSteed::new(self.first_day_gan_index() as usize, self.first_day_zhi_index() as usize)
    }
    pub fn get_kitchen_god_steed(&self) -> KitchenGodSteed {
        self.kitchen_god_steed()
    }

    pub fn year_fortunes(&self) -> Vec<YearFortune> {
        vec![
            self.tou_liang_info(),
            self.cao_zi_info(),
            self.geng_tian_info(),
            self.hua_shou_info(),
            self.zhi_shui_info(),
            self.tuo_gu_info(),
            self.qiang_mi_info(),
            self.kan_can_info(),
            self.gong_zhu_info(),
            self.jia_tian_info(),
            self.fen_bing_info(),
            self.de_jin_info(),
            self.ren_bing_info(),
            self.ren_chu_info(),
        ]
    }
}

impl CalendarSpan for LunarYear {
    fn first_solar_day(&self) -> Solar {
        LunarYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        LunarYear::last_solar_day(self)
    }
}
