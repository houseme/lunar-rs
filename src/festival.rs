//! tyme4rs-style festival wrapper objects.

use std::fmt;

use crate::event::{CalendarKind, Event, EventKind, EventSource};
use crate::{JieQi, Lunar, LunarMonth, Solar};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SolarFestivalRule {
    name: &'static str,
    month: i32,
    day: i32,
    start_year: i32,
}

const SOLAR_FESTIVALS: &[SolarFestivalRule] = &[
    SolarFestivalRule { name: "元旦", month: 1, day: 1, start_year: 1950 },
    SolarFestivalRule { name: "妇女节", month: 3, day: 8, start_year: 1950 },
    SolarFestivalRule { name: "植树节", month: 3, day: 12, start_year: 1979 },
    SolarFestivalRule { name: "劳动节", month: 5, day: 1, start_year: 1950 },
    SolarFestivalRule { name: "青年节", month: 5, day: 4, start_year: 1949 },
    SolarFestivalRule { name: "儿童节", month: 6, day: 1, start_year: 1950 },
    SolarFestivalRule { name: "建党节", month: 7, day: 1, start_year: 1941 },
    SolarFestivalRule { name: "建军节", month: 8, day: 1, start_year: 1933 },
    SolarFestivalRule { name: "教师节", month: 9, day: 10, start_year: 1985 },
    SolarFestivalRule { name: "国庆节", month: 10, day: 1, start_year: 1949 },
];

/// 公历现代节日对象。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SolarFestival {
    index: usize,
    day: Solar,
}

impl SolarFestival {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Option<Self> {
        SOLAR_FESTIVALS.iter().enumerate().find_map(|(index, rule)| {
            if year < rule.start_year || month != rule.month || day != rule.day {
                return None;
            }
            Solar::from_ymd(year, month, day).ok().map(|day| Self { index, day })
        })
    }

    pub fn from_index(year: i32, index: usize) -> Option<Self> {
        let rule = SOLAR_FESTIVALS.get(index)?;
        if year < rule.start_year {
            return None;
        }
        Solar::from_ymd(year, rule.month, rule.day).ok().map(|day| Self { index, day })
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn get_index(&self) -> usize {
        self.index()
    }

    pub fn name(&self) -> &'static str {
        SOLAR_FESTIVALS[self.index].name
    }

    pub fn get_name(&self) -> String {
        self.name().to_string()
    }

    pub const fn day(&self) -> Solar {
        self.day
    }

    pub const fn get_day(&self) -> Solar {
        self.day()
    }

    pub fn get_start_year(&self) -> i32 {
        SOLAR_FESTIVALS[self.index].start_year
    }

    pub fn next(&self, n: i32) -> Option<Self> {
        let size = SOLAR_FESTIVALS.len() as i32;
        let raw_index = self.index as i32 + n;
        let year = self.day.year() + raw_index.div_euclid(size);
        let index = raw_index.rem_euclid(size) as usize;
        Self::from_index(year, index)
    }

    pub fn to_event(&self) -> Event {
        Event::with_meta(
            EventKind::SolarFestival,
            CalendarKind::Solar,
            EventSource::BuiltInFestival,
            self.name(),
            self.day,
            Some(format!("index={} start_year={}", self.index(), self.get_start_year())),
            30,
            Some(format!("solar-festival:{}:{}", self.day.to_ymd(), self.name())),
            true,
            true,
            vec!["solar".to_string(), "festival".to_string(), "tyme_compat".to_string()],
        )
    }
}

impl fmt::Display for SolarFestival {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day, self.name())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LunarFestivalRule {
    LunarDay { name: &'static str, month: i32, day: i32 },
    SolarTerm { name: &'static str, term: &'static str },
    NewYearEve,
}

const LUNAR_FESTIVALS: &[LunarFestivalRule] = &[
    LunarFestivalRule::LunarDay { name: "春节", month: 1, day: 1 },
    LunarFestivalRule::LunarDay { name: "元宵节", month: 1, day: 15 },
    LunarFestivalRule::LunarDay { name: "龙头节", month: 2, day: 2 },
    LunarFestivalRule::LunarDay { name: "上巳节", month: 3, day: 3 },
    LunarFestivalRule::SolarTerm { name: "清明节", term: "清明" },
    LunarFestivalRule::LunarDay { name: "端午节", month: 5, day: 5 },
    LunarFestivalRule::LunarDay { name: "七夕节", month: 7, day: 7 },
    LunarFestivalRule::LunarDay { name: "中元节", month: 7, day: 15 },
    LunarFestivalRule::LunarDay { name: "中秋节", month: 8, day: 15 },
    LunarFestivalRule::LunarDay { name: "重阳节", month: 9, day: 9 },
    LunarFestivalRule::SolarTerm { name: "冬至节", term: "冬至" },
    LunarFestivalRule::LunarDay { name: "腊八节", month: 12, day: 8 },
    LunarFestivalRule::NewYearEve,
];

/// 农历传统节日对象。
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LunarFestival {
    index: usize,
    year: i32,
    month: i32,
    day: i32,
    solar: Solar,
}

impl LunarFestival {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Option<Self> {
        let lunar = Lunar::from_ymd(year, month, day).ok()?;
        LUNAR_FESTIVALS.iter().enumerate().find_map(|(index, _)| {
            Self::from_index(year, index).filter(|festival| {
                festival.year == lunar.year() && festival.month == lunar.month() && festival.day == lunar.day()
            })
        })
    }

    pub fn from_index(year: i32, index: usize) -> Option<Self> {
        let rule = *LUNAR_FESTIVALS.get(index)?;
        let lunar = match rule {
            LunarFestivalRule::LunarDay { month, day, .. } => Lunar::from_ymd(year, month, day).ok()?,
            LunarFestivalRule::SolarTerm { term, .. } => JieQi::from_name(year, term)?.solar_day().lunar(),
            LunarFestivalRule::NewYearEve => {
                let month = LunarMonth::from_ym(year, 12)?;
                Lunar::from_ymd(year, 12, month.day_count()).ok()?
            }
        };
        Some(Self { index, year: lunar.year(), month: lunar.month(), day: lunar.day(), solar: lunar.solar() })
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn get_index(&self) -> usize {
        self.index()
    }

    pub fn name(&self) -> &'static str {
        match LUNAR_FESTIVALS[self.index] {
            LunarFestivalRule::LunarDay { name, .. } | LunarFestivalRule::SolarTerm { name, .. } => name,
            LunarFestivalRule::NewYearEve => "除夕",
        }
    }

    pub fn get_name(&self) -> String {
        self.name().to_string()
    }

    pub fn day(&self) -> Lunar {
        Lunar::from_ymd(self.year, self.month, self.day).unwrap_or_else(|_| self.solar.lunar())
    }

    pub fn get_day(&self) -> Lunar {
        self.day()
    }

    pub const fn solar(&self) -> Solar {
        self.solar
    }

    pub fn get_solar_term(&self) -> Option<JieQi> {
        match LUNAR_FESTIVALS[self.index] {
            LunarFestivalRule::SolarTerm { term, .. } => JieQi::from_name(self.solar.year(), term),
            LunarFestivalRule::LunarDay { .. } | LunarFestivalRule::NewYearEve => None,
        }
    }

    pub fn next(&self, n: i32) -> Option<Self> {
        let size = LUNAR_FESTIVALS.len() as i32;
        let raw_index = self.index as i32 + n;
        let year = self.year + raw_index.div_euclid(size);
        let index = raw_index.rem_euclid(size) as usize;
        Self::from_index(year, index)
    }

    pub fn to_event(&self) -> Event {
        Event::with_meta(
            EventKind::LunarFestival,
            CalendarKind::Lunar,
            EventSource::BuiltInFestival,
            self.name(),
            self.solar,
            Some(format!("index={} lunar={}", self.index(), self.day())),
            50,
            Some(format!("lunar-festival:{}:{}", self.solar.to_ymd(), self.name())),
            true,
            true,
            vec!["lunar".to_string(), "festival".to_string(), "tyme_compat".to_string()],
        )
    }
}

impl fmt::Display for LunarFestival {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.day(), self.name())
    }
}
