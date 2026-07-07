//! Strongly-typed culture/domain primitives.
//!
//! These types are the first step toward Phase 2 of the roadmap: replacing
//! raw string-heavy APIs with stable, composable domain objects.

use std::fmt;

use crate::lunar_util;
use crate::solar_util;

const DIRECTION_NAMES: [&str; 9] = ["北", "西南", "东", "东南", "中", "西北", "西", "东北", "南"];
const STEM_ELEMENTS: [&str; 10] = ["木", "木", "火", "火", "土", "土", "金", "金", "水", "水"];
const BRANCH_ELEMENTS: [&str; 12] = ["水", "土", "木", "木", "土", "火", "火", "土", "金", "金", "土", "水"];
const ELEMENT_NAMES: [&str; 5] = ["木", "火", "土", "金", "水"];
const ELEMENT_DIRECTIONS: [(&str, &str); 5] = [("木", "东"), ("火", "南"), ("土", "中"), ("金", "西"), ("水", "北")];
const FETUS_HEAVEN_STEM_NAMES: [&str; 5] = ["门", "碓磨", "厨灶", "仓库", "房床"];
const FETUS_EARTH_BRANCH_NAMES: [&str; 6] = ["碓", "厕", "炉", "门", "栖", "床"];
const MINOR_REN_NAMES: [&str; 6] = ["大安", "留连", "速喜", "赤口", "小吉", "空亡"];
const MINOR_REN_ELEMENTS: [&str; 6] = ["木", "水", "火", "金", "木", "土"];
const NINE_NAMES: [&str; 9] = ["一九", "二九", "三九", "四九", "五九", "六九", "七九", "八九", "九九"];
const XIU_ANIMAL_NAMES: [&str; 28] = [
    "蛟", "龙", "貉", "兔", "狐", "虎", "豹", "獬", "牛", "蝠", "鼠", "燕", "猪", "獝", "狼", "狗", "彘", "鸡", "乌",
    "猴", "猿", "犴", "羊", "獐", "马", "鹿", "蛇", "蚓",
];
const BEAST_NAMES: [&str; 4] = ["青龙", "玄武", "白虎", "朱雀"];
const ZONE_NAMES: [&str; 4] = ["东", "北", "西", "南"];
const TERRAIN_NAMES: [&str; 12] = ["长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养"];
const LAND_NAMES: [&str; 9] = ["玄天", "朱天", "苍天", "阳天", "钧天", "幽天", "颢天", "变天", "炎天"];
const YUAN_CYCLE_NAMES: [&str; 3] = ["上元", "中元", "下元"];
const YUN_CYCLE_NAMES: [&str; 9] = ["一运", "二运", "三运", "四运", "五运", "六运", "七运", "八运", "九运"];
const SIX_STAR_NAMES: [&str; 6] = ["先胜", "友引", "先负", "佛灭", "大安", "赤口"];
const SEVEN_STAR_NAMES: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];
const ECLIPTIC_NAMES: [&str; 2] = ["黄道", "黑道"];
const TWELVE_STAR_NAMES: [&str; 12] =
    ["青龙", "明堂", "天刑", "朱雀", "金匮", "天德", "白虎", "玉堂", "天牢", "玄武", "司命", "勾陈"];
const TWELVE_STAR_ECLIPTIC_INDICES: [usize; 12] = [0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1];
const TEN_STAR_NAMES: [&str; 10] = ["比肩", "劫财", "食神", "伤官", "偏财", "正财", "七杀", "正官", "偏印", "正印"];
const KITCHEN_GOD_STEED_NUMBERS: [&str; 12] =
    ["一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二"];
const NAYIN_NAMES: [&str; 30] = [
    "海中金",
    "炉中火",
    "大林木",
    "路旁土",
    "剑锋金",
    "山头火",
    "涧下水",
    "城头土",
    "白蜡金",
    "杨柳木",
    "泉中水",
    "屋上土",
    "霹雳火",
    "松柏木",
    "长流水",
    "沙中金",
    "山下火",
    "平地木",
    "壁上土",
    "金箔金",
    "覆灯火",
    "天河水",
    "大驿土",
    "钗钏金",
    "桑柘木",
    "大溪水",
    "沙中土",
    "天上火",
    "石榴木",
    "大海水",
];

pub trait NamedCulture {
    fn name(&self) -> &str;
}

pub trait CycleItem: NamedCulture + Copy {
    fn from_cycle_index(index: usize) -> Self;

    fn index(&self) -> usize;

    fn size() -> usize;

    fn next(&self, offset: isize) -> Self {
        let size = Self::size() as isize;
        let index = (self.index() as isize + offset).rem_euclid(size) as usize;
        Self::from_cycle_index(index)
    }

    fn steps_to(&self, target_index: usize) -> usize {
        (target_index + Self::size() - self.index()) % Self::size()
    }

    fn steps_back_to(&self, target_index: usize) -> isize {
        let size = Self::size() as isize;
        -((self.index() as isize - target_index as isize + size) % size)
    }

    fn steps_close_to(&self, target_index: usize) -> isize {
        let forward = self.steps_to(target_index) as isize;
        let backward = self.steps_back_to(target_index);
        if forward <= backward.abs() { forward } else { backward }
    }
}

pub trait CultureDay: NamedCulture {
    fn day_index(&self) -> Option<i32>;

    fn is_boundary(&self) -> bool {
        self.day_index().is_none()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Direction {
    name: &'static str,
}

impl Direction {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn from_index(index: usize) -> Self {
        Self { name: DIRECTION_NAMES[index % DIRECTION_NAMES.len()] }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        DIRECTION_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        DIRECTION_NAMES.iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::direction(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Element {
    name: &'static str,
}

impl Element {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn from_index(index: usize) -> Self {
        Self { name: ELEMENT_NAMES[index % ELEMENT_NAMES.len()] }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        ELEMENT_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        ELEMENT_NAMES.iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub fn direction(&self) -> Direction {
        for (element, direction) in ELEMENT_DIRECTIONS {
            if element == self.name {
                return Direction::new(direction);
            }
        }
        Direction::new("")
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::wu_xing(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Zodiac {
    name: &'static str,
}

impl Zodiac {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn from_index(index: usize) -> Self {
        Self::new(lunar_util::tables::SHENG_XIAO[index % 12 + 1])
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::SHENG_XIAO[1..].iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        lunar_util::tables::SHENG_XIAO[1..].iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::sheng_xiao(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Constellation {
    index: usize,
}

impl Constellation {
    pub fn from_index(index: usize) -> Self {
        Self { index: index % solar_util::XINGZUO.len() }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        solar_util::XINGZUO.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        solar_util::XINGZUO[self.index]
    }

    pub fn next(&self, offset: isize) -> Self {
        let size = solar_util::XINGZUO.len() as isize;
        let index = (self.index as isize + offset).rem_euclid(size) as usize;
        Self::from_index(index)
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::constellation(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Duty {
    name: &'static str,
}

impl Duty {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub fn from_index(index: usize) -> Self {
        Self { name: lunar_util::tables::ZHI_XING[index % Self::size() + 1] }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::ZHI_XING[1..].iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        lunar_util::tables::ZHI_XING[1..].iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn size() -> usize {
        12
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::duty(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Phase {
    name: &'static str,
}

impl Phase {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::phase(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Phenology {
    term: String,
    three_hou: &'static str,
    wu_hou: &'static str,
}

impl Phenology {
    pub fn new(term: String, three_hou: &'static str, wu_hou: &'static str) -> Self {
        Self { term, three_hou, wu_hou }
    }

    pub fn term(&self) -> &str {
        &self.term
    }

    pub const fn three_hou(&self) -> &'static str {
        self.three_hou
    }

    pub const fn wu_hou(&self) -> &'static str {
        self.wu_hou
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SolarTermDay {
    name: String,
    day_index: i32,
}

impl SolarTermDay {
    pub fn new(name: impl Into<String>, day_index: i32) -> Self {
        Self { name: name.into(), day_index }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn day_index_value(&self) -> i32 {
        self.day_index
    }
}

impl fmt::Display for SolarTermDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name, self.day_index)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhenologyDay {
    phenology: Phenology,
    day_index: i32,
}

impl PhenologyDay {
    pub fn new(phenology: Phenology, day_index: i32) -> Self {
        Self { phenology, day_index }
    }

    pub const fn phenology(&self) -> &Phenology {
        &self.phenology
    }

    pub const fn day_index_value(&self) -> i32 {
        self.day_index
    }

    pub const fn name(&self) -> &'static str {
        self.phenology.wu_hou()
    }
}

impl fmt::Display for PhenologyDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name(), self.day_index)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PhaseDay {
    phase: Phase,
    day_index: i32,
}

impl PhaseDay {
    pub const fn new(phase: Phase, day_index: i32) -> Self {
        Self { phase, day_index }
    }

    pub const fn phase(&self) -> Phase {
        self.phase
    }

    pub const fn day_index_value(&self) -> i32 {
        self.day_index
    }

    pub fn name(&self) -> &'static str {
        self.phase.name()
    }
}

impl fmt::Display for PhaseDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name(), self.day_index)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HeavenStem {
    index: usize,
}

impl HeavenStem {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::GAN
            .iter()
            .position(|value| *value == name)
            .and_then(|index| if index == 0 { None } else { Some(Self::from_index(index - 1)) })
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        lunar_util::tables::GAN[self.index + 1]
    }

    pub fn element(&self) -> Element {
        Element::new(STEM_ELEMENTS[self.index])
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::gan(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EarthBranch {
    index: usize,
}

impl EarthBranch {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::ZHI
            .iter()
            .position(|value| *value == name)
            .and_then(|index| if index == 0 { None } else { Some(Self::from_index(index - 1)) })
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        lunar_util::tables::ZHI[self.index + 1]
    }

    pub fn zodiac(&self) -> Zodiac {
        Zodiac::new(lunar_util::tables::SHENG_XIAO[self.index + 1])
    }

    pub fn element(&self) -> Element {
        Element::new(BRANCH_ELEMENTS[self.index])
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> &'static str {
        crate::i18n::zhi(self.name(), language)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SixtyCycle {
    index: usize,
}

impl SixtyCycle {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::JIA_ZI.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        lunar_util::tables::JIA_ZI[self.index]
    }

    pub fn heaven_stem(&self) -> HeavenStem {
        HeavenStem::from_index(self.index % 10)
    }

    pub fn earth_branch(&self) -> EarthBranch {
        EarthBranch::from_index(self.index % 12)
    }

    pub fn next(&self, offset: isize) -> Self {
        let size = lunar_util::tables::JIA_ZI.len() as isize;
        let index = (self.index as isize + offset).rem_euclid(size) as usize;
        Self::from_index(index)
    }

    #[cfg(feature = "i18n")]
    pub fn name_in_lang(&self, language: crate::i18n::Language) -> String {
        crate::i18n::ganzhi(self.heaven_stem().name(), self.earth_branch().name(), language)
    }
}

macro_rules! define_sixty_cycle_pillar {
    ($ty:ident) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $ty {
            cycle: SixtyCycle,
        }

        impl $ty {
            pub const fn new(cycle: SixtyCycle) -> Self {
                Self { cycle }
            }

            pub fn from_name(name: &str) -> Option<Self> {
                SixtyCycle::from_name(name).map(Self::new)
            }

            pub const fn cycle(&self) -> SixtyCycle {
                self.cycle
            }

            pub fn name(&self) -> &'static str {
                self.cycle.name()
            }

            pub fn heaven_stem(&self) -> HeavenStem {
                self.cycle.heaven_stem()
            }

            pub fn earth_branch(&self) -> EarthBranch {
                self.cycle.earth_branch()
            }

            pub fn nayin(&self) -> Nayin {
                Nayin::new(lunar_util::nayin(self.name()))
            }
        }

        impl NamedCulture for $ty {
            fn name(&self) -> &str {
                self.name()
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }
    };
}

define_sixty_cycle_pillar!(SixtyCycleYear);
define_sixty_cycle_pillar!(SixtyCycleMonth);
define_sixty_cycle_pillar!(SixtyCycleDay);
define_sixty_cycle_pillar!(SixtyCycleHour);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ThreePillars {
    year: SixtyCycleYear,
    month: SixtyCycleMonth,
    day: SixtyCycleDay,
}

impl ThreePillars {
    pub const fn new(year: SixtyCycleYear, month: SixtyCycleMonth, day: SixtyCycleDay) -> Self {
        Self { year, month, day }
    }

    pub const fn year(&self) -> SixtyCycleYear {
        self.year
    }

    pub const fn month(&self) -> SixtyCycleMonth {
        self.month
    }

    pub const fn day(&self) -> SixtyCycleDay {
        self.day
    }
}

impl fmt::Display for ThreePillars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.year, self.month, self.day)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GodLuck {
    Auspicious,
    Inauspicious,
}

impl GodLuck {
    pub const fn from_index(index: usize) -> Self {
        if index % 2 == 0 { Self::Auspicious } else { Self::Inauspicious }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "吉" => Some(Self::Auspicious),
            "凶" => Some(Self::Inauspicious),
            _ => None,
        }
    }

    pub const fn index(&self) -> usize {
        match self {
            Self::Auspicious => 0,
            Self::Inauspicious => 1,
        }
    }

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Auspicious => "吉",
            Self::Inauspicious => "凶",
        }
    }

    pub const fn name(&self) -> &'static str {
        self.label()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct God {
    name: String,
    luck: GodLuck,
}

impl God {
    pub fn new(name: impl Into<String>, luck: GodLuck) -> Self {
        Self { name: name.into(), luck }
    }

    pub fn from_index(index: usize) -> Self {
        let index = index % Self::size();
        let luck = if index < 60 { GodLuck::Auspicious } else { GodLuck::Inauspicious };
        Self::new(lunar_util::tables::SHEN_SHA[index], luck)
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::SHEN_SHA.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> Option<usize> {
        lunar_util::tables::SHEN_SHA.iter().position(|value| *value == self.name())
    }

    pub fn next(&self, offset: isize) -> Option<Self> {
        let index = self.index()?;
        let size = Self::size() as isize;
        let next_index = (index as isize + offset).rem_euclid(size) as usize;
        Some(Self::from_index(next_index))
    }

    pub const fn size() -> usize {
        lunar_util::tables::SHEN_SHA.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn luck(&self) -> GodLuck {
        self.luck
    }

    pub const fn is_auspicious(&self) -> bool {
        matches!(self.luck, GodLuck::Auspicious)
    }

    pub const fn is_inauspicious(&self) -> bool {
        matches!(self.luck, GodLuck::Inauspicious)
    }
}

impl fmt::Display for God {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TabooKind {
    Recommend,
    Avoid,
}

impl TabooKind {
    pub fn from_index(index: usize) -> Self {
        match index % 2 {
            0 => Self::Recommend,
            _ => Self::Avoid,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "宜" => Some(Self::Recommend),
            "忌" => Some(Self::Avoid),
            _ => None,
        }
    }

    pub const fn index(&self) -> usize {
        match self {
            Self::Recommend => 0,
            Self::Avoid => 1,
        }
    }

    pub const fn label(&self) -> &'static str {
        match self {
            Self::Recommend => "宜",
            Self::Avoid => "忌",
        }
    }

    pub const fn name(&self) -> &'static str {
        self.label()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Taboo {
    name: String,
    kind: TabooKind,
}

impl Taboo {
    pub fn new(name: impl Into<String>, kind: TabooKind) -> Self {
        Self { name: name.into(), kind }
    }

    pub fn from_index(index: usize, kind: TabooKind) -> Self {
        Self::new(lunar_util::tables::YI_JI[index % Self::size()], kind)
    }

    pub fn from_name(name: &str, kind: TabooKind) -> Option<Self> {
        lunar_util::tables::YI_JI.iter().position(|value| *value == name).map(|index| Self::from_index(index, kind))
    }

    pub fn index(&self) -> Option<usize> {
        lunar_util::tables::YI_JI.iter().position(|value| *value == self.name())
    }

    pub fn next(&self, offset: isize) -> Option<Self> {
        let index = self.index()?;
        let size = Self::size() as isize;
        let next_index = (index as isize + offset).rem_euclid(size) as usize;
        Some(Self::from_index(next_index, self.kind()))
    }

    pub const fn size() -> usize {
        lunar_util::tables::YI_JI.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn kind(&self) -> TabooKind {
        self.kind
    }

    pub const fn is_recommend(&self) -> bool {
        matches!(self.kind, TabooKind::Recommend)
    }

    pub const fn is_avoid(&self) -> bool {
        matches!(self.kind, TabooKind::Avoid)
    }
}

impl fmt::Display for Taboo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PengZuHeavenStem {
    index: usize,
}

impl PengZuHeavenStem {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::PENGZU_GAN
            .iter()
            .position(|value| *value == name)
            .and_then(|index| if index == 0 { None } else { Some(Self::from_index(index - 1)) })
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        lunar_util::tables::PENGZU_GAN[self.index + 1]
    }
}

impl fmt::Display for PengZuHeavenStem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PengZuEarthBranch {
    index: usize,
}

impl PengZuEarthBranch {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::PENGZU_ZHI
            .iter()
            .position(|value| *value == name)
            .and_then(|index| if index == 0 { None } else { Some(Self::from_index(index - 1)) })
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        lunar_util::tables::PENGZU_ZHI[self.index + 1]
    }
}

impl fmt::Display for PengZuEarthBranch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PengZu {
    heaven_stem: &'static str,
    earth_branch: &'static str,
}

impl PengZu {
    pub const fn new(heaven_stem: &'static str, earth_branch: &'static str) -> Self {
        Self { heaven_stem, earth_branch }
    }

    pub const fn heaven_stem(&self) -> &'static str {
        self.heaven_stem
    }

    pub const fn earth_branch(&self) -> &'static str {
        self.earth_branch
    }

    pub fn heaven_stem_item(&self) -> PengZuHeavenStem {
        PengZuHeavenStem::from_name(self.heaven_stem).unwrap_or_else(|| PengZuHeavenStem::from_index(0))
    }

    pub fn earth_branch_item(&self) -> PengZuEarthBranch {
        PengZuEarthBranch::from_name(self.earth_branch).unwrap_or_else(|| PengZuEarthBranch::from_index(0))
    }

    pub fn items(&self) -> [&'static str; 2] {
        [self.heaven_stem, self.earth_branch]
    }
}

impl fmt::Display for PengZu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.heaven_stem, self.earth_branch)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TianShenType {
    YellowPath,
    BlackPath,
}

impl TianShenType {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::YellowPath => "黄道",
            Self::BlackPath => "黑道",
        }
    }

    pub const fn luck(&self) -> GodLuck {
        match self {
            Self::YellowPath => GodLuck::Auspicious,
            Self::BlackPath => GodLuck::Inauspicious,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TianShen {
    name: &'static str,
    kind: TianShenType,
}

impl TianShen {
    pub fn new(name: &'static str) -> Self {
        let kind = match lunar_util::tian_shen_type(name) {
            "黄道" => TianShenType::YellowPath,
            _ => TianShenType::BlackPath,
        };
        Self { name, kind }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn kind(&self) -> TianShenType {
        self.kind
    }

    pub const fn luck(&self) -> GodLuck {
        self.kind.luck()
    }
}

impl fmt::Display for TianShen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XiuAnimal {
    name: &'static str,
}

impl XiuAnimal {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn from_index(index: usize) -> Self {
        Self { name: XIU_ANIMAL_NAMES[index % XIU_ANIMAL_NAMES.len()] }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        XIU_ANIMAL_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        XIU_ANIMAL_NAMES.iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for XiuAnimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Shou {
    name: &'static str,
}

impl Shou {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn from_index(index: usize) -> Self {
        Self { name: BEAST_NAMES[index % BEAST_NAMES.len()] }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        BEAST_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        BEAST_NAMES.iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for Shou {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

pub type Beast = Shou;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Zone {
    index: usize,
}

impl Zone {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        ZONE_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        ZONE_NAMES[self.index % ZONE_NAMES.len()]
    }

    pub fn direction(&self) -> Direction {
        Direction::new(self.name())
    }

    pub fn beast(&self) -> Beast {
        Beast::from_index(self.index)
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Terrain {
    index: usize,
}

impl Terrain {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        TERRAIN_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        TERRAIN_NAMES[self.index % TERRAIN_NAMES.len()]
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Land {
    index: usize,
}

impl Land {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        LAND_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        LAND_NAMES[self.index % LAND_NAMES.len()]
    }

    pub const fn direction(&self) -> Direction {
        Direction::from_index(self.index)
    }
}

impl fmt::Display for Land {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Week {
    index: usize,
}

impl Week {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        solar_util::WEEK.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        solar_util::WEEK[self.index % solar_util::WEEK.len()]
    }
}

impl fmt::Display for Week {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Xiu {
    name: &'static str,
    luck: GodLuck,
    zheng: &'static str,
    animal: XiuAnimal,
    gong: Direction,
    shou: Shou,
}

impl Xiu {
    pub fn new(name: &'static str) -> Self {
        let luck = match lunar_util::xiu_luck(name) {
            "吉" => GodLuck::Auspicious,
            _ => GodLuck::Inauspicious,
        };
        let zheng = lunar_util::zheng(name);
        let animal = XiuAnimal::new(lunar_util::animal(name));
        let gong = Direction::new(lunar_util::gong(name));
        let shou = Shou::new(lunar_util::shou(gong.name()));

        Self { name, luck, zheng, animal, gong, shou }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn luck(&self) -> GodLuck {
        self.luck
    }

    pub const fn zheng(&self) -> &'static str {
        self.zheng
    }

    pub const fn animal(&self) -> XiuAnimal {
        self.animal
    }

    pub const fn gong(&self) -> Direction {
        self.gong
    }

    pub fn zone(&self) -> Zone {
        Zone::from_name(self.gong.name()).unwrap_or_else(|| Zone::from_index(0))
    }

    pub const fn shou(&self) -> Shou {
        self.shou
    }
}

impl fmt::Display for Xiu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lu {
    mutual: &'static str,
    advancing: Option<&'static str>,
}

impl Lu {
    pub const fn new(mutual: &'static str, advancing: Option<&'static str>) -> Self {
        Self { mutual, advancing }
    }

    pub const fn mutual(&self) -> &'static str {
        self.mutual
    }

    pub const fn advancing(&self) -> Option<&'static str> {
        self.advancing
    }

    pub const fn has_advancing(&self) -> bool {
        self.advancing.is_some()
    }
}

impl fmt::Display for Lu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.mutual)?;
        f.write_str("命互禄")?;
        if let Some(advancing) = self.advancing {
            write!(f, " {advancing}命进禄")?;
        }
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChongSha {
    gan: &'static str,
    branch: &'static str,
    zodiac: Zodiac,
    sha: Direction,
}

impl ChongSha {
    pub const fn new(gan: &'static str, branch: &'static str, zodiac: Zodiac, sha: Direction) -> Self {
        Self { gan, branch, zodiac, sha }
    }

    pub const fn gan(&self) -> &'static str {
        self.gan
    }

    pub const fn branch(&self) -> &'static str {
        self.branch
    }

    pub const fn zodiac(&self) -> Zodiac {
        self.zodiac
    }

    pub const fn sha(&self) -> Direction {
        self.sha
    }
}

impl fmt::Display for ChongSha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{}){}", self.gan, self.branch, self.zodiac.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct XunKong {
    name: &'static str,
}

impl XunKong {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub fn branches(&self) -> Option<(EarthBranch, EarthBranch)> {
        let mut chars = self.name.chars();
        let first = chars.next()?;
        let second = chars.next()?;
        let first = EarthBranch::from_name(&first.to_string())?;
        let second = EarthBranch::from_name(&second.to_string())?;
        Some((first, second))
    }
}

impl fmt::Display for XunKong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Xun {
    name: &'static str,
    kong: XunKong,
}

impl Xun {
    pub fn new(name: &'static str, kong: &'static str) -> Self {
        Self { name, kong: XunKong::new(kong) }
    }

    pub fn from_index(index: usize) -> Self {
        let index = index % lunar_util::tables::XUN.len();
        Self::new(lunar_util::tables::XUN[index], lunar_util::tables::XUN_KONG[index])
    }

    pub fn from_name(name: &str) -> Option<Self> {
        lunar_util::tables::XUN.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        lunar_util::tables::XUN.iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn kong(&self) -> XunKong {
        self.kong
    }
}

impl fmt::Display for Xun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaiSuiPosition {
    direction: Direction,
    description: &'static str,
}

impl TaiSuiPosition {
    pub const fn new(direction: Direction, description: &'static str) -> Self {
        Self { direction, description }
    }

    pub const fn direction(&self) -> Direction {
        self.direction
    }

    pub const fn description(&self) -> &'static str {
        self.description
    }
}

impl fmt::Display for TaiSuiPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.direction.name(), self.description)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TaiPosition {
    name: &'static str,
}

impl TaiPosition {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for TaiPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FetusHeavenStem {
    index: usize,
}

impl FetusHeavenStem {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        FETUS_HEAVEN_STEM_NAMES[self.index % FETUS_HEAVEN_STEM_NAMES.len()]
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FetusEarthBranch {
    index: usize,
}

impl FetusEarthBranch {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        FETUS_EARTH_BRANCH_NAMES[self.index % FETUS_EARTH_BRANCH_NAMES.len()]
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FetusDay {
    cycle: SixtyCycleDay,
    heaven_stem: FetusHeavenStem,
    earth_branch: FetusEarthBranch,
    position: TaiPosition,
}

impl FetusDay {
    pub fn new(cycle: SixtyCycleDay) -> Self {
        let stem_index = cycle.heaven_stem().index() % FETUS_HEAVEN_STEM_NAMES.len();
        let branch_index = cycle.earth_branch().index() % FETUS_EARTH_BRANCH_NAMES.len();
        let position = TaiPosition::new(lunar_util::tables::POSITION_TAI_DAY[cycle.cycle().index()]);
        Self {
            cycle,
            heaven_stem: FetusHeavenStem::from_index(stem_index),
            earth_branch: FetusEarthBranch::from_index(branch_index),
            position,
        }
    }

    pub const fn cycle(&self) -> SixtyCycleDay {
        self.cycle
    }

    pub const fn heaven_stem(&self) -> FetusHeavenStem {
        self.heaven_stem
    }

    pub const fn earth_branch(&self) -> FetusEarthBranch {
        self.earth_branch
    }

    pub const fn position(&self) -> TaiPosition {
        self.position
    }

    pub fn name(&self) -> &str {
        self.position.name()
    }
}

impl fmt::Display for FetusDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FetusMonth {
    month_index: usize,
    position: TaiPosition,
}

impl FetusMonth {
    pub fn from_month(month: i32) -> Option<Self> {
        if !(1..=12).contains(&month) {
            return None;
        }
        let month_index = (month - 1) as usize;
        Some(Self { month_index, position: TaiPosition::new(lunar_util::tables::POSITION_TAI_MONTH[month_index]) })
    }

    pub const fn month_index(&self) -> usize {
        self.month_index
    }

    pub const fn position(&self) -> TaiPosition {
        self.position
    }

    pub fn name(&self) -> &str {
        self.position.name()
    }
}

impl fmt::Display for FetusMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MinorRen {
    index: usize,
}

impl MinorRen {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        MINOR_REN_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        MINOR_REN_NAMES[self.index % MINOR_REN_NAMES.len()]
    }

    pub fn luck(&self) -> GodLuck {
        if self.index % 2 == 0 { GodLuck::Auspicious } else { GodLuck::Inauspicious }
    }

    pub fn element(&self) -> Element {
        Element::new(MINOR_REN_ELEMENTS[self.index % MINOR_REN_ELEMENTS.len()])
    }
}

impl fmt::Display for MinorRen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nine {
    index: usize,
}

impl Nine {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        NINE_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        NINE_NAMES[self.index % NINE_NAMES.len()]
    }
}

impl fmt::Display for Nine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NineDay {
    nine: Nine,
    day_index: i32,
}

impl NineDay {
    pub const fn new(nine: Nine, day_index: i32) -> Self {
        Self { nine, day_index }
    }

    pub const fn nine(&self) -> Nine {
        self.nine
    }

    pub const fn day_index_value(&self) -> i32 {
        self.day_index
    }

    pub fn name(&self) -> &'static str {
        self.nine.name()
    }
}

impl fmt::Display for NineDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name(), self.day_index)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HideHeavenStemType {
    Residual,
    Middle,
    Main,
}

impl HideHeavenStemType {
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Residual),
            1 => Some(Self::Middle),
            2 => Some(Self::Main),
            _ => None,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "余气" => Some(Self::Residual),
            "中气" => Some(Self::Middle),
            "本气" => Some(Self::Main),
            _ => None,
        }
    }

    pub const fn index(&self) -> usize {
        match self {
            Self::Residual => 0,
            Self::Middle => 1,
            Self::Main => 2,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Residual => "余气",
            Self::Middle => "中气",
            Self::Main => "本气",
        }
    }
}

impl fmt::Display for HideHeavenStemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HideHeavenStem {
    heaven_stem: HeavenStem,
    kind: HideHeavenStemType,
}

impl HideHeavenStem {
    pub const fn new(heaven_stem: HeavenStem, kind: HideHeavenStemType) -> Self {
        Self { heaven_stem, kind }
    }

    pub fn from_index(heaven_stem_index: usize, kind: HideHeavenStemType) -> Self {
        Self::new(HeavenStem::from_index(heaven_stem_index % 10), kind)
    }

    pub fn from_name(heaven_stem_name: &str, kind: HideHeavenStemType) -> Option<Self> {
        HeavenStem::from_name(heaven_stem_name).map(|heaven_stem| Self::new(heaven_stem, kind))
    }

    pub const fn heaven_stem(&self) -> HeavenStem {
        self.heaven_stem
    }

    pub const fn kind(&self) -> HideHeavenStemType {
        self.kind
    }

    pub fn element(&self) -> Element {
        self.heaven_stem.element()
    }

    pub fn name(&self) -> &'static str {
        self.heaven_stem.name()
    }
}

impl fmt::Display for HideHeavenStem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HideHeavenStemDay {
    hide_heaven_stem: HideHeavenStem,
    day_index: i32,
    name: String,
}

impl HideHeavenStemDay {
    pub fn new(hide_heaven_stem: HideHeavenStem, day_index: i32) -> Self {
        let name = format!("{}{}", hide_heaven_stem.name(), hide_heaven_stem.element().name());
        Self { hide_heaven_stem, day_index, name }
    }

    pub const fn hide_heaven_stem(&self) -> HideHeavenStem {
        self.hide_heaven_stem
    }

    pub const fn day_index_value(&self) -> i32 {
        self.day_index
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for HideHeavenStemDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name(), self.day_index)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Nayin {
    name: &'static str,
}

impl Nayin {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn from_index(index: usize) -> Self {
        Self { name: NAYIN_NAMES[index % NAYIN_NAMES.len()] }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        NAYIN_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub fn index(&self) -> usize {
        NAYIN_NAMES.iter().position(|value| *value == self.name).unwrap_or(0)
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub fn element(&self) -> Element {
        match self.name.chars().last() {
            Some('木') => Element::new("木"),
            Some('火') => Element::new("火"),
            Some('土') => Element::new("土"),
            Some('金') => Element::new("金"),
            Some('水') => Element::new("水"),
            _ => Element::new(""),
        }
    }
}

impl fmt::Display for Nayin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Season {
    name: &'static str,
}

impl Season {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LiuYao {
    name: &'static str,
}

impl LiuYao {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl fmt::Display for LiuYao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct YuanCycle {
    index: usize,
}

impl YuanCycle {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self::from_name(name.as_ref()).unwrap_or_else(|| Self::from_index(0))
    }

    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        YUAN_CYCLE_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        YUAN_CYCLE_NAMES[self.index % YUAN_CYCLE_NAMES.len()]
    }
}

impl fmt::Display for YuanCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct YunCycle {
    index: usize,
}

impl YunCycle {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self::from_name(name.as_ref()).unwrap_or_else(|| Self::from_index(0))
    }

    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        YUN_CYCLE_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        YUN_CYCLE_NAMES[self.index % YUN_CYCLE_NAMES.len()]
    }

    pub const fn yuan_cycle(&self) -> YuanCycle {
        YuanCycle::from_index((self.index % YUN_CYCLE_NAMES.len()) / 3)
    }
}

impl fmt::Display for YunCycle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SixStar {
    index: usize,
}

impl SixStar {
    pub const fn from_index(index: usize) -> Self {
        Self { index: index % SIX_STAR_NAMES.len() }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        SIX_STAR_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        SIX_STAR_NAMES[self.index % SIX_STAR_NAMES.len()]
    }
}

impl fmt::Display for SixStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SevenStar {
    index: usize,
}

impl SevenStar {
    pub const fn from_index(index: usize) -> Self {
        Self { index: index % SEVEN_STAR_NAMES.len() }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        SEVEN_STAR_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        SEVEN_STAR_NAMES[self.index % SEVEN_STAR_NAMES.len()]
    }
}

impl fmt::Display for SevenStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ecliptic {
    index: usize,
}

impl Ecliptic {
    pub const fn from_index(index: usize) -> Self {
        Self { index: index % ECLIPTIC_NAMES.len() }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        ECLIPTIC_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        ECLIPTIC_NAMES[self.index % ECLIPTIC_NAMES.len()]
    }

    pub const fn luck(&self) -> GodLuck {
        GodLuck::from_index(self.index)
    }
}

impl fmt::Display for Ecliptic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TwelveStar {
    index: usize,
}

impl TwelveStar {
    pub const fn from_index(index: usize) -> Self {
        Self { index: index % TWELVE_STAR_NAMES.len() }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        TWELVE_STAR_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        TWELVE_STAR_NAMES[self.index % TWELVE_STAR_NAMES.len()]
    }

    pub const fn ecliptic(&self) -> Ecliptic {
        Ecliptic::from_index(TWELVE_STAR_ECLIPTIC_INDICES[self.index])
    }
}

impl fmt::Display for TwelveStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TenStar {
    index: usize,
}

impl TenStar {
    pub const fn from_index(index: usize) -> Self {
        Self { index: index % TEN_STAR_NAMES.len() }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        TEN_STAR_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &'static str {
        TEN_STAR_NAMES[self.index % TEN_STAR_NAMES.len()]
    }
}

impl fmt::Display for TenStar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KitchenGodSteed {
    first_day_heaven_stem_index: usize,
    first_day_earth_branch_index: usize,
}

impl KitchenGodSteed {
    pub const fn new(first_day_heaven_stem_index: usize, first_day_earth_branch_index: usize) -> Self {
        Self {
            first_day_heaven_stem_index: first_day_heaven_stem_index % 10,
            first_day_earth_branch_index: first_day_earth_branch_index % 12,
        }
    }

    pub const fn from_first_day(heaven_stem: HeavenStem, earth_branch: EarthBranch) -> Self {
        Self::new(heaven_stem.index(), earth_branch.index())
    }

    pub const fn name(&self) -> &'static str {
        "灶马头"
    }

    pub const fn first_day_heaven_stem_index(&self) -> usize {
        self.first_day_heaven_stem_index
    }

    pub const fn first_day_earth_branch_index(&self) -> usize {
        self.first_day_earth_branch_index
    }

    fn by_heaven_stem(&self, target_index: usize) -> &'static str {
        KITCHEN_GOD_STEED_NUMBERS[(target_index + 10 - self.first_day_heaven_stem_index) % 10]
    }

    fn by_earth_branch(&self, target_index: usize) -> &'static str {
        KITCHEN_GOD_STEED_NUMBERS[(target_index + 12 - self.first_day_earth_branch_index) % 12]
    }

    pub fn mouse(&self) -> String {
        format!("{}鼠偷粮", self.by_earth_branch(0))
    }

    pub fn grass(&self) -> String {
        format!("草子{}分", self.by_earth_branch(0))
    }

    pub fn cattle(&self) -> String {
        format!("{}牛耕田", self.by_earth_branch(1))
    }

    pub fn flower(&self) -> String {
        format!("花收{}分", self.by_earth_branch(3))
    }

    pub fn dragon(&self) -> String {
        format!("{}龙治水", self.by_earth_branch(4))
    }

    pub fn horse(&self) -> String {
        format!("{}马驮谷", self.by_earth_branch(6))
    }

    pub fn chicken(&self) -> String {
        format!("{}鸡抢米", self.by_earth_branch(9))
    }

    pub fn silkworm(&self) -> String {
        format!("{}姑看蚕", self.by_earth_branch(9))
    }

    pub fn pig(&self) -> String {
        format!("{}屠共猪", self.by_earth_branch(11))
    }

    pub fn field(&self) -> String {
        format!("甲田{}分", self.by_heaven_stem(0))
    }

    pub fn cake(&self) -> String {
        format!("{}人分饼", self.by_heaven_stem(2))
    }

    pub fn gold(&self) -> String {
        format!("{}日得金", self.by_heaven_stem(7))
    }

    pub fn people_cakes(&self) -> String {
        format!("{}人{}丙", self.by_earth_branch(2), self.by_heaven_stem(2))
    }

    pub fn people_hoes(&self) -> String {
        format!("{}人{}锄", self.by_earth_branch(2), self.by_heaven_stem(3))
    }
}

impl fmt::Display for KitchenGodSteed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum YearFortuneKind {
    TouLiang,
    CaoZi,
    GengTian,
    HuaShou,
    ZhiShui,
    TuoGu,
    QiangMi,
    KanCan,
    GongZhu,
    JiaTian,
    FenBing,
    DeJin,
    RenBing,
    RenChu,
}

impl YearFortuneKind {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::TouLiang => "鼠偷粮",
            Self::CaoZi => "草子",
            Self::GengTian => "牛耕田",
            Self::HuaShou => "花收",
            Self::ZhiShui => "龙治水",
            Self::TuoGu => "马驮谷",
            Self::QiangMi => "鸡抢米",
            Self::KanCan => "姑看蚕",
            Self::GongZhu => "屠共猪",
            Self::JiaTian => "甲田",
            Self::FenBing => "人分饼",
            Self::DeJin => "日得金",
            Self::RenBing => "人几丙",
            Self::RenChu => "人几锄",
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct YearFortune {
    kind: YearFortuneKind,
    text: String,
}

impl YearFortune {
    pub fn new(kind: YearFortuneKind, text: impl Into<String>) -> Self {
        Self { kind, text: text.into() }
    }

    pub const fn kind(&self) -> YearFortuneKind {
        self.kind
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl fmt::Display for YearFortune {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DogDay {
    name: String,
    day_index: i32,
}

impl DogDay {
    pub fn new(name: impl Into<String>, day_index: i32) -> Self {
        Self { name: name.into(), day_index }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn day_index(&self) -> i32 {
        self.day_index
    }
}

impl fmt::Display for DogDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}第{}天", self.name, self.day_index)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlumRainKind {
    Entering,
    Leaving,
}

impl PlumRainKind {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Entering => "入梅",
            Self::Leaving => "出梅",
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlumRainDay {
    kind: PlumRainKind,
    day_index: Option<i32>,
}

impl PlumRainDay {
    pub const fn entering(day_index: i32) -> Self {
        Self { kind: PlumRainKind::Entering, day_index: Some(day_index) }
    }

    pub const fn leaving() -> Self {
        Self { kind: PlumRainKind::Leaving, day_index: None }
    }

    pub const fn kind(&self) -> PlumRainKind {
        self.kind
    }

    pub const fn name(&self) -> &'static str {
        self.kind.name()
    }

    pub const fn day_index(&self) -> Option<i32> {
        self.day_index
    }
}

impl fmt::Display for PlumRainDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.day_index {
            Some(day_index) => write!(f, "{}第{}天", self.name(), day_index),
            None => f.write_str(self.name()),
        }
    }
}

macro_rules! impl_named_culture {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl NamedCulture for $ty {
                fn name(&self) -> &str {
                    self.name()
                }
            }
        )+
    };
}

impl_named_culture!(
    Direction,
    Element,
    Zodiac,
    Constellation,
    Duty,
    Phase,
    HeavenStem,
    EarthBranch,
    SixtyCycle,
    GodLuck,
    God,
    TabooKind,
    Taboo,
    PengZuHeavenStem,
    PengZuEarthBranch,
    TianShen,
    XiuAnimal,
    Shou,
    Zone,
    Terrain,
    Land,
    Week,
    KitchenGodSteed,
    Xiu,
    XunKong,
    Xun,
    TaiPosition,
    FetusHeavenStem,
    FetusEarthBranch,
    FetusDay,
    FetusMonth,
    MinorRen,
    Nine,
    NineDay,
    HideHeavenStem,
    HideHeavenStemDay,
    Nayin,
    Season,
    SolarTermDay,
    PhenologyDay,
    PhaseDay,
    LiuYao,
    YuanCycle,
    YunCycle,
    SixStar,
    SevenStar,
    Ecliptic,
    TwelveStar,
    TenStar,
    DogDay,
    PlumRainDay,
);

impl CycleItem for Constellation {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index)
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        solar_util::XINGZUO.len()
    }
}

impl CycleItem for Direction {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        DIRECTION_NAMES.len()
    }
}

impl CycleItem for Element {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        ELEMENT_NAMES.len()
    }
}

impl CycleItem for Zodiac {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index)
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        12
    }
}

impl CycleItem for Duty {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        lunar_util::tables::ZHI_XING.len() - 1
    }
}

impl CycleItem for HeavenStem {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        10
    }
}

impl CycleItem for EarthBranch {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        12
    }
}

impl CycleItem for SixtyCycle {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        60
    }
}

impl CycleItem for GodLuck {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        2
    }
}

impl CycleItem for TabooKind {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        2
    }
}

impl CycleItem for SixStar {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        SIX_STAR_NAMES.len()
    }
}

impl CycleItem for SevenStar {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        SEVEN_STAR_NAMES.len()
    }
}

impl CycleItem for Ecliptic {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        ECLIPTIC_NAMES.len()
    }
}

impl CycleItem for TwelveStar {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        TWELVE_STAR_NAMES.len()
    }
}

impl CycleItem for TenStar {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        TEN_STAR_NAMES.len()
    }
}

impl CycleItem for Xun {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        lunar_util::tables::XUN.len()
    }
}

impl CycleItem for PengZuHeavenStem {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        lunar_util::tables::PENGZU_GAN.len() - 1
    }
}

impl CycleItem for PengZuEarthBranch {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        lunar_util::tables::PENGZU_ZHI.len() - 1
    }
}

impl CycleItem for XiuAnimal {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        XIU_ANIMAL_NAMES.len()
    }
}

impl CycleItem for Shou {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        BEAST_NAMES.len()
    }
}

impl CycleItem for Zone {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        ZONE_NAMES.len()
    }
}

impl CycleItem for Terrain {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        TERRAIN_NAMES.len()
    }
}

impl CycleItem for Land {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        LAND_NAMES.len()
    }
}

impl CycleItem for YuanCycle {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        YUAN_CYCLE_NAMES.len()
    }
}

impl CycleItem for YunCycle {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        YUN_CYCLE_NAMES.len()
    }
}

impl CycleItem for Week {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        solar_util::WEEK.len()
    }
}

impl CycleItem for FetusHeavenStem {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        FETUS_HEAVEN_STEM_NAMES.len()
    }
}

impl CycleItem for FetusEarthBranch {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        FETUS_EARTH_BRANCH_NAMES.len()
    }
}

impl CycleItem for MinorRen {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        MINOR_REN_NAMES.len()
    }
}

impl CycleItem for Nine {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        NINE_NAMES.len()
    }
}

impl CycleItem for Nayin {
    fn from_cycle_index(index: usize) -> Self {
        Self::from_index(index % Self::size())
    }

    fn index(&self) -> usize {
        self.index()
    }

    fn size() -> usize {
        NAYIN_NAMES.len()
    }
}

impl CultureDay for DogDay {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index())
    }
}

impl CultureDay for PlumRainDay {
    fn day_index(&self) -> Option<i32> {
        self.day_index()
    }
}

impl CultureDay for SolarTermDay {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index_value())
    }
}

impl CultureDay for PhenologyDay {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index_value())
    }
}

impl CultureDay for PhaseDay {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index_value())
    }
}

impl CultureDay for NineDay {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index_value())
    }
}

impl CultureDay for HideHeavenStemDay {
    fn day_index(&self) -> Option<i32> {
        Some(self.day_index_value())
    }
}
