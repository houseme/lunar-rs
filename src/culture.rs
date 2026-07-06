//! Strongly-typed culture/domain primitives.
//!
//! These types are the first step toward Phase 2 of the roadmap: replacing
//! raw string-heavy APIs with stable, composable domain objects.

use crate::lunar_util;

const STEM_ELEMENTS: [&str; 10] = ["木", "木", "火", "火", "土", "土", "金", "金", "水", "水"];
const BRANCH_ELEMENTS: [&str; 12] = ["水", "土", "木", "木", "土", "火", "火", "土", "金", "金", "土", "水"];
const ELEMENT_DIRECTIONS: [(&str, &str); 5] = [("木", "东"), ("火", "南"), ("土", "中"), ("金", "西"), ("水", "北")];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Direction {
    name: &'static str,
}

impl Direction {
    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &'static str {
        self.name
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

    pub const fn name(&self) -> &'static str {
        self.name
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

    pub const fn name(&self) -> &'static str {
        self.name
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
}
