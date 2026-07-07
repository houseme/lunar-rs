//! RabByung / Tibetan calendar.
//!
//! This Phase 4 slice ports the year/month/day layers needed for core
//! conversion and validation. The supported conversion window follows the
//! reference month-day table: `1950..=2050`.

use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

use crate::event::{Event, EventQuery};
use crate::multi_calendar::{
    CalendarDay, CalendarSpan, point_all_events, point_events, point_events_until, point_find_events,
    point_find_events_until, span_all_events, span_contains_solar, span_events, span_events_until, span_find_events,
    span_find_events_until,
};
use crate::{LunarError, Solar, Zodiac};

const ELEMENT_NAMES: [&str; 5] = ["木", "火", "土", "铁", "水"];
const ZODIAC_NAMES: [&str; 12] = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];
const MONTH_NAMES: [&str; 12] =
    ["正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"];
const MONTH_ALIASES: [&str; 12] = [
    "神变月",
    "苦行月",
    "具香月",
    "萨嘎月",
    "作净月",
    "明净月",
    "具醉月",
    "具贤月",
    "天降月",
    "持众月",
    "庄严月",
    "满意月",
];
const DAY_NAMES: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二", "十三", "十四",
    "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四", "廿五", "廿六", "廿七", "廿八",
    "廿九", "三十",
];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RabByungElement {
    index: usize,
}

impl RabByungElement {
    pub const fn from_index(index: usize) -> Self {
        Self { index }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        ELEMENT_NAMES.iter().position(|value| *value == name).map(Self::from_index)
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn name(&self) -> &'static str {
        ELEMENT_NAMES[self.index]
    }
}

impl fmt::Display for RabByungElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RabByungYear {
    rab_byung_index: usize,
    element_index: usize,
    zodiac_index: usize,
}

impl RabByungYear {
    pub fn from_year(year: i32) -> Result<Self, LunarError> {
        if !(1027..=9999).contains(&year) {
            return Err(LunarError::Parse(format!("illegal rab-byung year: {year}")));
        }

        let sixty_cycle_index = (year - 4).rem_euclid(60) as usize;
        let zodiac_index = sixty_cycle_index % 12;
        let element_index = ((sixty_cycle_index % 10) / 2) % 5;

        Ok(Self { rab_byung_index: ((year - 1024) / 60) as usize, element_index, zodiac_index })
    }

    pub const fn rab_byung_index(&self) -> usize {
        self.rab_byung_index
    }

    pub fn year(&self) -> i32 {
        1024 + self.rab_byung_index as i32 * 60 + self.sixty_cycle_index() as i32
    }

    pub const fn element(&self) -> RabByungElement {
        RabByungElement::from_index(self.element_index)
    }

    pub fn zodiac(&self) -> Zodiac {
        Zodiac::new(ZODIAC_NAMES[self.zodiac_index])
    }

    pub fn leap_month(&self) -> usize {
        let mut y = 1_i32;
        let mut m = 4_i32;
        let mut t = 1_i32;
        let current_year = self.year();

        while y < current_year {
            let i = m + 31 + t;
            y += 2;
            m = i - 23;
            if i > 35 {
                y += 1;
                m -= 12;
            }
            t = 1 - t;
        }

        if y == current_year { m as usize } else { 0 }
    }

    pub fn month_count(&self) -> usize {
        if self.leap_month() > 0 { 13 } else { 12 }
    }

    pub fn next(&self, years: i32) -> Result<Self, LunarError> {
        Self::from_year(self.year() + years)
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        self.year() == solar.year()
    }

    pub fn first_day(&self) -> RabByungDay {
        self.first_month().first_day()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_month(&self) -> RabByungMonth {
        self.months().last().copied().unwrap()
    }

    pub fn last_day(&self) -> RabByungDay {
        self.last_month().last_day()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn events(&self) -> Vec<Event> {
        span_events(self)
    }

    pub fn all_events(&self) -> Vec<Event> {
        span_all_events(self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events(self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        span_events_until(self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events_until(self, end, query)
    }

    fn sixty_cycle_index(&self) -> usize {
        let zodiac_index = self.zodiac_index as i32;
        (6 * (self.element_index as i32 * 2 + zodiac_index.rem_euclid(2)) - 5 * zodiac_index).rem_euclid(60) as usize
    }

    fn chinese_number(mut n: usize) -> String {
        let digits = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
        let units = ["", "十", "百"];
        let mut out = String::new();
        let mut pos = 0;

        while n > 0 {
            let digit = n % 10;
            if digit > 0 {
                out = format!("{}{}{}", digits[digit], units[pos], out);
            } else if !out.is_empty() {
                out = format!("{}{}", digits[digit], out);
            }
            n /= 10;
            pos += 1;
        }

        if out.starts_with("一十") { out.chars().skip(1).collect() } else { out }
    }

    pub fn first_month(&self) -> RabByungMonth {
        RabByungMonth::from_ym(self.year(), 1).unwrap()
    }

    pub fn months(&self) -> Vec<RabByungMonth> {
        let leap_month = self.leap_month() as i32;
        let mut months = Vec::new();
        for month in 1..=12 {
            months.push(RabByungMonth::from_ym(self.year(), month).unwrap());
            if leap_month == month {
                months.push(RabByungMonth::from_ym(self.year(), -month).unwrap());
            }
        }
        months
    }
}

impl CalendarSpan for RabByungYear {
    fn first_solar_day(&self) -> Solar {
        RabByungYear::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        RabByungYear::last_solar_day(self)
    }
}

impl fmt::Display for RabByungYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "第{}饶迥{}{}年",
            Self::chinese_number(self.rab_byung_index + 1),
            self.element(),
            self.zodiac().name()
        )
    }
}

const MONTH_DAY_DATA: &str = include_str!("rab_byung_month_days.txt");

static RAB_BYUNG_MONTH_DAYS: LazyLock<HashMap<i32, Vec<i32>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let years = MONTH_DAY_DATA.trim().split(',');
    let mut year = 1950_i32;
    let mut month_index = 11_i32;

    for segment in years {
        let mut remain = segment;
        while !remain.is_empty() {
            let mut chars = remain.chars();
            let len = (chars.next().unwrap() as i32 - '0' as i32) as usize;
            let mut data = Vec::with_capacity(len);
            for _ in 0..len {
                data.push(chars.next().unwrap() as i32 - '5' as i32 - 30);
            }
            map.insert(year * 13 + month_index, data);
            month_index += 1;
            remain = &remain[1 + len..];
        }
        year += 1;
        month_index = 0;
    }

    map
});

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RabByungMonth {
    year: i32,
    month: i32,
    leap: bool,
}

impl RabByungMonth {
    pub fn from_ym(year: i32, month: i32) -> Result<Self, LunarError> {
        if month == 0 || !(-12..=12).contains(&month) {
            return Err(LunarError::Parse(format!("illegal rab-byung month: {month}")));
        }
        if !(1950..=2050).contains(&year) {
            return Err(LunarError::Parse(format!("rab-byung year {year} must between 1950 and 2050")));
        }

        let leap = month < 0;
        let month_abs = month.abs();
        if year == 1950 && month_abs < 12 {
            return Err(LunarError::Parse(format!("month {month} must be 12 in rab-byung year {year}")));
        }

        let rab_byung_year = RabByungYear::from_year(year)?;
        if leap && month_abs as usize != rab_byung_year.leap_month() {
            return Err(LunarError::Parse(format!("illegal leap month {month_abs} in rab-byung year {year}")));
        }

        Ok(Self { year, month: month_abs, leap })
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month
    }

    pub const fn is_leap(&self) -> bool {
        self.leap
    }

    pub fn month_with_leap(&self) -> i32 {
        if self.leap { -self.month } else { self.month }
    }

    pub fn rab_byung_year(&self) -> RabByungYear {
        RabByungYear::from_year(self.year).unwrap()
    }

    pub fn index_in_year(&self) -> usize {
        let mut index = (self.month - 1) as usize;
        if self.leap {
            index += 1;
        } else {
            let leap_month = self.rab_byung_year().leap_month();
            if leap_month > 0 && self.month as usize > leap_month {
                index += 1;
            }
        }
        index
    }

    pub fn name(&self) -> String {
        let name = MONTH_NAMES[(self.month - 1) as usize];
        if self.leap { format!("闰{name}") } else { name.to_string() }
    }

    pub fn alias(&self) -> String {
        let alias = MONTH_ALIASES[(self.month - 1) as usize];
        if self.leap { format!("闰{alias}") } else { alias.to_string() }
    }

    pub fn next(&self, months: i32) -> Result<Self, LunarError> {
        if months == 0 {
            return Ok(*self);
        }

        let mut index = self.index_in_year() as i32 + 1 + months;
        let mut year = self.rab_byung_year();
        if months > 0 {
            let mut month_count = year.month_count() as i32;
            while index > month_count {
                index -= month_count;
                year = year.next(1)?;
                month_count = year.month_count() as i32;
            }
        } else {
            while index <= 0 {
                year = year.next(-1)?;
                index += year.month_count() as i32;
            }
        }

        let mut leap = false;
        let leap_month = year.leap_month() as i32;
        let mut month = index;
        if leap_month > 0 {
            if month == leap_month + 1 {
                leap = true;
            }
            if month > leap_month {
                month -= 1;
            }
        }

        Self::from_ym(year.year(), if leap { -month } else { month })
    }

    pub fn special_days(&self) -> Vec<i32> {
        let key = self.year * 13 + self.index_in_year() as i32;
        RAB_BYUNG_MONTH_DAYS.get(&key).cloned().unwrap_or_default()
    }

    pub fn leap_days(&self) -> Vec<i32> {
        self.special_days().into_iter().filter(|day| *day > 0).collect()
    }

    pub fn miss_days(&self) -> Vec<i32> {
        self.special_days().into_iter().filter(|day| *day < 0).collect()
    }

    pub fn day_count(&self) -> i32 {
        30 + self.leap_days().len() as i32 - self.miss_days().len() as i32
    }

    pub fn first_day(&self) -> RabByungDay {
        RabByungDay::from_ymd(self.year, self.month_with_leap(), 1).unwrap()
    }

    pub fn first_solar_day(&self) -> Solar {
        self.first_day().solar()
    }

    pub fn last_day(&self) -> RabByungDay {
        self.days().last().copied().unwrap()
    }

    pub fn last_solar_day(&self) -> Solar {
        self.last_day().solar()
    }

    pub fn days(&self) -> Vec<RabByungDay> {
        let miss_days = self.miss_days();
        let leap_days = self.leap_days();
        let mut out = Vec::new();
        for day in 1..=30 {
            if miss_days.contains(&(-day)) {
                continue;
            }
            out.push(RabByungDay::from_ymd(self.year, self.month_with_leap(), day).unwrap());
            if leap_days.contains(&day) {
                out.push(RabByungDay::from_ymd(self.year, self.month_with_leap(), -day).unwrap());
            }
        }
        out
    }

    pub fn contains_solar(&self, solar: Solar) -> bool {
        span_contains_solar(self, solar)
    }

    pub fn contains_day(&self, day: RabByungDay) -> bool {
        day.rab_byung_month() == *self
    }

    pub fn events(&self) -> Vec<Event> {
        span_events(self)
    }

    pub fn all_events(&self) -> Vec<Event> {
        span_all_events(self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events(self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        span_events_until(self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        span_find_events_until(self, end, query)
    }
}

impl CalendarSpan for RabByungMonth {
    fn first_solar_day(&self) -> Solar {
        RabByungMonth::first_solar_day(self)
    }

    fn last_solar_day(&self) -> Solar {
        RabByungMonth::last_solar_day(self)
    }
}

impl fmt::Display for RabByungMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rab_byung_year(), self.name())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RabByungDay {
    year: i32,
    month: i32,
    day: i32,
    leap_day: bool,
}

impl RabByungDay {
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self, LunarError> {
        if day == 0 || !(-30..=30).contains(&day) {
            return Err(LunarError::Parse(format!("illegal rab-byung day {day} in {month}")));
        }

        let month_obj = RabByungMonth::from_ym(year, month)?;
        let leap_day = day < 0;
        let day_abs = day.abs();

        if leap_day && !month_obj.leap_days().contains(&day_abs) {
            return Err(LunarError::Parse(format!("illegal leap day {day_abs} in {month}")));
        }
        if !leap_day && month_obj.miss_days().contains(&(-day_abs)) {
            return Err(LunarError::Parse(format!("illegal day {day_abs} in {month}")));
        }

        Ok(Self { year, month, day: day_abs, leap_day })
    }

    pub fn from_solar(solar: Solar) -> Result<Self, LunarError> {
        let mut days = solar.subtract(&Solar::from_ymd(1951, 1, 8).unwrap());
        let mut month = RabByungMonth::from_ym(1950, 12)?;
        let mut count = month.day_count();
        while days >= count {
            days -= count;
            month = month.next(1)?;
            count = month.day_count();
        }

        let mut day = days + 1;
        for special in month.special_days() {
            if special < 0 {
                if day >= -special {
                    day += 1;
                }
            } else if special > 0 {
                if day == special + 1 {
                    day = -special;
                    break;
                } else if day > special + 1 {
                    day -= 1;
                }
            }
        }

        Self::from_ymd(month.year(), month.month_with_leap(), day)
    }

    pub const fn year(&self) -> i32 {
        self.year
    }

    pub const fn month(&self) -> i32 {
        self.month.abs()
    }

    pub const fn day(&self) -> i32 {
        self.day
    }

    pub const fn is_leap(&self) -> bool {
        self.leap_day
    }

    pub fn day_with_leap(&self) -> i32 {
        if self.leap_day { -self.day } else { self.day }
    }

    pub fn rab_byung_month(&self) -> RabByungMonth {
        RabByungMonth::from_ym(self.year, self.month).unwrap()
    }

    pub fn name(&self) -> String {
        let name = DAY_NAMES[(self.day - 1) as usize];
        if self.leap_day { format!("闰{name}") } else { name.to_string() }
    }

    pub fn next(&self, days: i32) -> Result<Self, LunarError> {
        if days == 0 { Ok(*self) } else { Self::from_solar(self.solar().next_day(days)) }
    }

    pub fn is_before(&self, other: Self) -> bool {
        self.solar().is_before(&other.solar())
    }

    pub fn is_after(&self, other: Self) -> bool {
        self.solar().is_after(&other.solar())
    }

    pub fn solar(&self) -> Solar {
        let mut month = RabByungMonth::from_ym(1950, 12).unwrap();
        let current_month = self.rab_byung_month();
        let mut days = 0_i32;
        while month != current_month {
            days += month.day_count();
            month = month.next(1).unwrap();
        }

        let mut day = self.day;
        for special in month.special_days() {
            if special < 0 {
                if day > -special {
                    day -= 1;
                }
            } else if special > 0 && day > special {
                day += 1;
            }
        }
        if self.leap_day {
            day += 1;
        }

        Solar::from_ymd(1951, 1, 7).unwrap().next_day(days + day)
    }

    pub fn subtract(&self, other: Self) -> i32 {
        self.solar().subtract(&other.solar())
    }

    pub fn events(&self) -> Vec<Event> {
        point_events(self)
    }

    pub fn all_events(&self) -> Vec<Event> {
        point_all_events(self)
    }

    pub fn find_events(&self, query: &EventQuery<'_>) -> Vec<Event> {
        point_find_events(self, query)
    }

    pub fn events_until(&self, end: Solar) -> Vec<Event> {
        point_events_until(self, end)
    }

    pub fn find_events_until(&self, end: Solar, query: &EventQuery<'_>) -> Vec<Event> {
        point_find_events_until(self, end, query)
    }
}

impl CalendarDay for RabByungDay {
    fn solar(&self) -> Solar {
        RabByungDay::solar(self)
    }
}

impl fmt::Display for RabByungDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rab_byung_month(), self.name())
    }
}
