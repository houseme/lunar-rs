//! 佛历工具。对应 lunar-go `FotoUtil/FotoUtil.go`。

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::key_index::{month_day_key, parse_month_day_key};

include!("foto_util_data.rs");
include!("foto_util_maps.rs");

static FESTIVAL_INDEX: LazyLock<HashMap<i32, Vec<Vec<&'static str>>>> = LazyLock::new(|| {
    FESTIVAL.iter().filter_map(|(key, value)| parse_month_day_key(key).map(|parsed| (parsed, value.clone()))).collect()
});

static OTHER_FESTIVAL_INDEX: LazyLock<HashMap<i32, Vec<&'static str>>> = LazyLock::new(|| {
    OTHER_FESTIVAL
        .iter()
        .filter_map(|(key, value)| parse_month_day_key(key).map(|parsed| (parsed, value.clone())))
        .collect()
});

static DAY_ZHAI_GUAN_YIN_INDEX: LazyLock<HashSet<i32>> =
    LazyLock::new(|| DAY_ZHAI_GUAN_YIN.iter().filter_map(|key| parse_month_day_key(key)).collect());

/// 当日印度星宿（27 宿）。
pub fn get_xiu(month: i32, day: i32) -> &'static str {
    let m = month.unsigned_abs() as usize;
    let idx = (XIU_OFFSET[m - 1] + i64::from(day) - 1).rem_euclid(27) as usize;
    XIU_27[idx]
}

pub fn festivals(month: i32, day: i32) -> &'static [Vec<&'static str>] {
    FESTIVAL_INDEX.get(&month_day_key(month, day)).map(Vec::as_slice).unwrap_or(&[])
}

pub fn other_festivals(month: i32, day: i32) -> &'static [&'static str] {
    OTHER_FESTIVAL_INDEX.get(&month_day_key(month, day)).map(Vec::as_slice).unwrap_or(&[])
}

pub fn is_day_zhai_guan_yin(month: i32, day: i32) -> bool {
    DAY_ZHAI_GUAN_YIN_INDEX.contains(&month_day_key(month, day))
}
