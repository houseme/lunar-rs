//! # lunar-rs
//!
//! 高性能中国农历 / 阳历 / 佛历 / 道历 / 八字 / 九星日历库。
//!
//! 基于 [寿星天文历](http://www.nongli.net/) 的高精度天文算法（VSOP87 / ELP 级数），
//! 参考实现为 [lunar-javascript](https://github.com/6tail/lunar-javascript) 与
//! [lunar-go](https://github.com/6tail/lunar-go)。
//!
//! ## 特性
//!
//! - 公历（阳历）/ 农历（阴历）互转，支持闰月、1582 历法改革、跨年边界。
//! - 二十四节气、节令（节气 / 中气），精度到秒。
//! - 干支（天干地支）、生肖、纳音、五行，三种年起算约定（正月初一 / 立春日 / 立春时刻）。
//! - 八字（四柱）、十神、藏干、十二长生（地势）、旬空、胎元 / 命宫 / 身宫。
//! - 大运 / 流年 / 流月 / 小运（运程）。
//! - 九星（九宫飞星：玄空 / 北斗 / 奇门 / 太乙）。
//! - 每日宜忌、吉神宜趋、凶煞宜忌、彭祖百忌、吉神方位、冲煞、建除十二值星、二十八宿、月相、数九、三伏、七十二候。
//! - 佛历、道历。
//! - 法定节假日（含调休）与薪资倍率。
//!
//! ## 快速示例
//!
//! ```
//! use lunar_rs::{Solar, Lunar};
//!
//! // 阳历 -> 农历
//! let solar = Solar::from_ymd(2020, 5, 1).unwrap();
//! let lunar = solar.lunar();
//! assert_eq!(lunar.to_string(), "二〇二〇年四月初九");
//!
//! // 农历 -> 阳历
//! let lunar = Lunar::from_ymd(2020, 4, 9).unwrap();
//! assert_eq!(lunar.solar().to_ymd(), "2020-05-01");
//!
//! // 干支 / 生肖
//! let lunar = Solar::from_ymd(2019, 5, 1).unwrap().lunar();
//! assert_eq!(lunar.year_in_gan_zhi(), "己亥");
//! assert_eq!(lunar.year_sheng_xiao(), "猪");
//! ```

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value
)]

#[doc(hidden)]
pub mod differential_support;
mod error;
#[cfg(feature = "i18n")]
pub mod i18n;
mod shou_xing;
pub mod solar_util;

pub mod lunar_util;

mod foto_util;
pub mod holiday_util;
mod nine_star_util;
mod tao_util;

mod culture;
mod eight_char;
mod event;
mod foto;
mod fu;
mod holiday;
mod jieqi;
mod lunar;
mod lunar_month;
mod lunar_time;
mod lunar_year;
mod nine_star;
mod shu_jiu;
mod solar;
mod solar_half_year;
mod solar_month;
mod solar_season;
mod solar_week;
mod solar_year;
mod tao;
mod yun;

pub use culture::{Direction, Duty, EarthBranch, Element, HeavenStem, Phase, Phenology, SixtyCycle, Zodiac};
pub use eight_char::EightChar;
pub use error::LunarError;
pub use event::{Event, EventKind};
pub use foto::Foto;
pub use fu::Fu;
pub use holiday::Holiday;
#[cfg(feature = "i18n")]
pub use i18n::Language;
pub use jieqi::JieQi;
pub use lunar::Lunar;
pub use lunar_month::LunarMonth;
pub use lunar_time::LunarTime;
pub use lunar_year::LunarYear;
pub use nine_star::NineStar;
pub use shu_jiu::ShuJiu;
pub use solar::Solar;
pub use solar_half_year::SolarHalfYear;
pub use solar_month::SolarMonth;
pub use solar_season::SolarSeason;
pub use solar_week::SolarWeek;
pub use solar_year::SolarYear;
pub use tao::Tao;
pub use yun::{DaYun, LiuNian, LiuYue, XiaoYun, Yun};

/// 性别：`1` 男，`0` 女（与参考实现 lunar-go 一致）。
pub type Gender = u8;

/// 起算约定：`1` 正月初一，`2` 立春当日（默认），`3` 立春时刻。
pub type Sect = u8;
