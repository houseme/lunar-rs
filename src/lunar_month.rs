//! 农历月。对应 lunar-go `calendar/LunarMonth.go`。

use crate::lunar_year::LunarYear;
use crate::lunar_util;
use crate::nine_star::NineStar;
use crate::solar::Solar;

/// 农历月（`month` 为负表示闰月）。
#[derive(Clone, Copy, Debug)]
pub struct LunarMonth {
    pub(crate) year: i32,
    pub(crate) month: i32,
    pub(crate) day_count: i32,
    pub(crate) index: i32,
    pub(crate) zhi_index: i64,
    pub(crate) first_julian_day: f64,
}

impl LunarMonth {
    pub(crate) const fn new(
        year: i32, month: i32, day_count: i32, first_julian_day: f64, index: i32,
    ) -> Self {
        let m = if month < 0 { -month } else { month };
        Self {
            year,
            month,
            day_count,
            first_julian_day,
            index,
            zhi_index: ((m - 1) as i64 + lunar_util::BASE_MONTH_ZHI_INDEX) % 12,
        }
    }

    /// 由年月查找（该年无此月返回 `None`）。
    pub fn from_ym(year: i32, month: i32) -> Option<LunarMonth> {
        LunarYear::from_year(year).get_month(month)
    }

    #[inline]
    pub const fn year(&self) -> i32 {
        self.year
    }
    #[inline]
    pub const fn month(&self) -> i32 {
        self.month
    }
    #[inline]
    pub const fn is_leap(&self) -> bool {
        self.month < 0
    }
    #[inline]
    pub const fn day_count(&self) -> i32 {
        self.day_count
    }
    #[inline]
    pub const fn first_julian_day(&self) -> f64 {
        self.first_julian_day
    }
    #[inline]
    pub const fn index(&self) -> i32 {
        self.index
    }
    #[inline]
    pub const fn zhi_index(&self) -> i64 {
        self.zhi_index
    }

    /// 月天干索引（年干遁月）。
    pub fn gan_index(&self) -> i64 {
        let offset = (LunarYear::from_year(self.year).gan_index() + 1) % 5 * 2;
        let m = if self.month < 0 { -self.month } else { self.month };
        ((m - 1) as i64 + offset) % 10
    }

    pub fn gan(&self) -> &'static str {
        lunar_util::tables::GAN[(self.gan_index() + 1) as usize]
    }
    pub fn zhi(&self) -> &'static str {
        lunar_util::tables::ZHI[(self.zhi_index + 1) as usize]
    }
    pub fn gan_zhi(&self) -> String {
        format!("{}{}", self.gan(), self.zhi())
    }

    pub fn position_xi(&self) -> &'static str {
        lunar_util::tables::POSITION_XI[(self.gan_index() + 1) as usize]
    }
    pub fn position_xi_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_xi())
    }
    pub fn position_yang_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YANG_GUI[(self.gan_index() + 1) as usize]
    }
    pub fn position_yang_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_yang_gui())
    }
    pub fn position_yin_gui(&self) -> &'static str {
        lunar_util::tables::POSITION_YIN_GUI[(self.gan_index() + 1) as usize]
    }
    pub fn position_yin_gui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_yin_gui())
    }
    pub fn position_fu(&self) -> &'static str {
        self.position_fu_by_sect(2)
    }
    pub fn position_fu_by_sect(&self, sect: u8) -> &'static str {
        let offset = (self.gan_index() + 1) as usize;
        if sect == 1 {
            lunar_util::tables::POSITION_FU[offset]
        } else {
            lunar_util::tables::POSITION_FU_2[offset]
        }
    }
    pub fn position_fu_desc(&self) -> &'static str {
        self.position_fu_desc_by_sect(2)
    }
    pub fn position_fu_desc_by_sect(&self, sect: u8) -> &'static str {
        lunar_util::position_desc(self.position_fu_by_sect(sect))
    }
    pub fn position_cai(&self) -> &'static str {
        lunar_util::tables::POSITION_CAI[(self.gan_index() + 1) as usize]
    }
    pub fn position_cai_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_cai())
    }

    /// 月太岁方位。
    pub fn position_tai_sui(&self) -> &'static str {
        let month = self.month.abs();
        let m = month % 4;
        match m {
            0 => "巽",
            1 => "艮",
            3 => "坤",
            _ => {
                let gan_index = Solar::from_julian_day(self.first_julian_day)
                    .lunar()
                    .month_gan_index();
                lunar_util::tables::POSITION_GAN[gan_index as usize]
            }
        }
    }
    pub fn position_tai_sui_desc(&self) -> &'static str {
        lunar_util::position_desc(self.position_tai_sui())
    }

    /// 月九星。
    pub fn nine_star(&self) -> NineStar {
        let index = LunarYear::from_year(self.year).zhi_index() % 3;
        let m = self.month.abs();
        let month_zhi_index = (13 + m) as i64 % 12;
        let mut n = 27 - index * 3;
        if month_zhi_index < lunar_util::BASE_MONTH_ZHI_INDEX {
            n -= 3;
        }
        let offset = (n - month_zhi_index) % 9;
        NineStar::from_index(offset)
    }

    /// 推进 / 回退 n 个月（跨年）。
    pub fn next(&self, n: i32) -> Option<LunarMonth> {
        if n == 0 {
            return LunarMonth::from_ym(self.year, self.month);
        }
        if n > 0 {
            let mut rest = n;
            let mut ny = self.year;
            let mut iy = self.year;
            let mut im = self.month;
            let mut index;
            let mut months = LunarYear::from_year(ny).months();
            loop {
                let mut found = 0;
                index = 0;
                for (i, m) in months.iter().enumerate() {
                    if m.year == iy && m.month == im {
                        index = i as i32;
                        found = 1;
                        break;
                    }
                }
                let _ = found;
                let size = months.len() as i32;
                let more = size - index - 1;
                if rest < more {
                    break;
                }
                rest -= more;
                let last = *months.last().unwrap();
                iy = last.year;
                im = last.month;
                ny += 1;
                months = LunarYear::from_year(ny).months();
            }
            let offset = (index + rest) as usize;
            months.get(offset).copied()
        } else {
            let mut rest = -n;
            let mut ny = self.year;
            let mut iy = self.year;
            let mut im = self.month;
            let mut index;
            let mut months = LunarYear::from_year(ny).months();
            loop {
                index = 0;
                for (i, m) in months.iter().enumerate() {
                    if m.year == iy && m.month == im {
                        index = i as i32;
                        break;
                    }
                }
                if rest <= index {
                    break;
                }
                rest -= index;
                let first = months[0];
                iy = first.year;
                im = first.month;
                ny -= 1;
                months = LunarYear::from_year(ny).months();
            }
            let offset = (index - rest) as usize;
            months.get(offset).copied()
        }
    }
}
