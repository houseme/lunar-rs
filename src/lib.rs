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
//! - 佛历、道历、回历、藏历、民国历、泰阳历、现代日本年号历、主体纪年、檀纪、儒略历、全新世纪年、拜占庭纪年、科普特历、埃塞俄比亚历、亚美尼亚历、罗马建城纪年、亚述纪年、西班牙纪元、印度国历、旧制泰佛历、法斯里历、纳纳克沙希历、塞琉古纪元、拉达那哥欣纪元、威尼斯纪年、鲁米历、光明纪年。
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

mod fasli;
mod foto_util;
pub mod holiday_util;
mod holocene;
mod nine_star_util;
mod tao_util;

mod anno_lucis;
mod armenian;
mod assyrian;
mod auc;
mod bengali;
mod byzantine;
mod coptic;
mod culture;
mod dangi;
mod eight_char;
mod ethiopian;
mod event;
mod foto;
mod fu;
mod hijri;
mod hispanic_era;
mod holiday;
mod japanese;
mod jd;
mod jieqi;
mod juche;
mod julian;
mod koki;
mod lunar;
mod lunar_month;
mod lunar_time;
mod lunar_week;
mod lunar_year;
mod minguo;
mod multi_calendar;
mod nanakshahi;
mod nine_star;
mod rab_byung;
mod rattanakosin;
mod rumi;
mod saka;
mod seleucid;
mod shu_jiu;
mod solar;
mod solar_half_year;
mod solar_month;
mod solar_season;
mod solar_week;
mod solar_year;
mod tao;
mod thai_buddhist;
mod thai_solar;
mod unit;
mod venetian;
mod yun;

pub use anno_lucis::{AnnoLucis, AnnoLucisMonth, AnnoLucisYear};
pub use armenian::{Armenian, ArmenianMonth, ArmenianYear};
pub use assyrian::{Assyrian, AssyrianMonth, AssyrianYear};
pub use auc::{Auc, AucMonth, AucYear};
pub use bengali::{Bengali, BengaliMonth, BengaliYear};
pub use byzantine::{Byzantine, ByzantineMonth, ByzantineYear};
pub use coptic::{Coptic, CopticMonth, CopticYear};
pub use culture::{
    Beast, ChongSha, Constellation, CultureDay, CycleItem, Dipper, Direction, DogDay, Duty, EarthBranch, Ecliptic,
    Element, FetusDay, FetusEarthBranch, FetusHeavenStem, FetusMonth, God, GodLuck, HeavenStem, HideHeavenStem,
    HideHeavenStemDay, HideHeavenStemType, KitchenGodSteed, Land, LiuYao, Lu, MinorRen, MoonPhase, MoonPhaseDay,
    NamedCulture, Nayin, Nine, NineDay, PengZu, PengZuEarthBranch, PengZuHeavenStem, Phase, PhaseDay, Phenology,
    PhenologyDay, PlumRainDay, PlumRainKind, Season, SevenStar, Shou, SixStar, SixtyCycle, SixtyCycleDay,
    SixtyCycleHour, SixtyCycleMonth, SixtyCycleYear, SolarTermDay, Taboo, TabooKind, TaiPosition, TaiSuiPosition,
    TenStar, Terrain, ThreePillars, TianShen, TianShenType, TwelveStar, Week, Xiu, XiuAnimal, Xun, XunKong,
    YearFortune, YearFortuneKind, YuanCycle, YunCycle, Zodiac, Zone,
};

pub type Animal = XiuAnimal;
pub type Luck = GodLuck;
pub type Sixty = YuanCycle;
pub type Sound = Nayin;
pub type Ten = Xun;
pub type Twenty = YunCycle;

pub use dangi::{Dangi, DangiMonth, DangiYear};
pub use eight_char::{
    DefaultEightCharProvider, EightChar, EightCharProvider, LunarSect1EightCharProvider, LunarSect2EightCharProvider,
};
pub use error::LunarError;
pub use ethiopian::{Ethiopian, EthiopianMonth, EthiopianYear};
pub use event::{
    CalendarKind, Event, EventBuilder, EventDayGroup, EventKind, EventManager, EventQuery, EventRangeKind, EventRule,
    EventSource, EventSourceFamily, EventType, EventWeekGroup, FotoFestivalEvent, HolidayEvent, HolidayPeriodEvent,
    JieQiEvent, LunarFestivalEvent, SolarFestivalEvent, TaoFestivalEvent, dedup_events, filter_events,
    group_event_days_by_week, group_events_by_day, scan_event_days_in_range, scan_event_days_in_range_filtered,
    scan_event_weeks_in_range, scan_event_weeks_in_range_filtered, scan_events_in_range, scan_events_in_range_filtered,
    sort_events,
};
pub use fasli::{Fasli, FasliMonth, FasliYear};
pub use foto::{Foto, FotoMonth, FotoYear};
pub use fu::Fu;
pub use hijri::{Hijri, HijriMonth, HijriYear};
pub use hispanic_era::{HispanicEra, HispanicEraMonth, HispanicEraYear};
pub use holiday::Holiday;
pub use holocene::{Holocene, HoloceneMonth, HoloceneYear};
#[cfg(feature = "i18n")]
pub use i18n::Language;
pub use japanese::{Japanese, JapaneseEra, JapaneseMonth, JapaneseYear};
pub use jd::JulianDay;
pub use jieqi::JieQi;
pub use juche::{Juche, JucheMonth, JucheYear};
pub use julian::{Julian, JulianMonth, JulianYear};
pub use koki::{Koki, KokiMonth, KokiYear};
pub use lunar::Lunar;
pub use lunar_month::LunarMonth;
pub use lunar_time::LunarTime;
pub use lunar_week::LunarWeek;
pub use lunar_year::LunarYear;
pub use minguo::{Minguo, MinguoMonth, MinguoYear};
pub use multi_calendar::{CalendarDay, CalendarSpan};
pub use nanakshahi::{Nanakshahi, NanakshahiMonth, NanakshahiYear};
pub use nine_star::NineStar;
pub use rab_byung::{RabByungDay, RabByungElement, RabByungMonth, RabByungYear};
pub use rattanakosin::{Rattanakosin, RattanakosinMonth, RattanakosinYear};
pub use rumi::{Rumi, RumiMonth, RumiYear};
pub use saka::{Saka, SakaMonth, SakaYear};
pub use seleucid::{Seleucid, SeleucidMonth, SeleucidYear};
pub use shu_jiu::ShuJiu;
pub use solar::Solar;
pub use solar_half_year::SolarHalfYear;
pub use solar_month::SolarMonth;
pub use solar_season::SolarSeason;
pub use solar_week::SolarWeek;
pub use solar_year::SolarYear;
pub use tao::{Tao, TaoMonth, TaoYear};
pub use thai_buddhist::{ThaiBuddhist, ThaiBuddhistMonth, ThaiBuddhistYear};
pub use thai_solar::{ThaiSolar, ThaiSolarMonth, ThaiSolarYear};
pub use unit::{DayUnit, MonthUnit, SecondUnit, WeekUnit, YearUnit};
pub use venetian::{Venetian, VenetianMonth, VenetianYear};
pub use yun::{
    ChildLimit, ChildLimitInfo, ChildLimitProvider, China95ChildLimitProvider, DecadeFortune,
    DefaultChildLimitProvider, Fortune, LunarSect1ChildLimitProvider, LunarSect2ChildLimitProvider,
};
pub use yun::{DaYun, LiuNian, LiuYue, XiaoYun, Yun};

pub type LegalHoliday = Holiday;
pub type LunarDay = Lunar;
pub type LunarHour<'a> = LunarTime<'a>;
pub type SolarDay = Solar;
pub type SolarTerm = JieQi;
pub type SolarTime = Solar;

/// 性别：`1` 男，`0` 女（与参考实现 lunar-go 一致）。
pub type Gender = u8;

/// 起算约定：`1` 正月初一，`2` 立春当日（默认），`3` 立春时刻。
pub type Sect = u8;
