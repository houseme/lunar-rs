//! 道历工具。对应 lunar-go `TaoUtil/TaoUtil.go`。

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::key_index::{month_day_key, parse_month_day_key};

include!("tao_util_data.rs");
include!("tao_util_maps.rs");

static FESTIVAL_INDEX: LazyLock<HashMap<i32, Vec<Vec<&'static str>>>> = LazyLock::new(|| {
    FESTIVAL.iter().filter_map(|(key, value)| parse_month_day_key(key).map(|parsed| (parsed, value.clone()))).collect()
});

static SAN_HUI_INDEX: LazyLock<HashSet<i32>> =
    LazyLock::new(|| SAN_HUI.iter().filter_map(|key| parse_month_day_key(key)).collect());
static SAN_YUAN_INDEX: LazyLock<HashSet<i32>> =
    LazyLock::new(|| SAN_YUAN.iter().filter_map(|key| parse_month_day_key(key)).collect());
static WU_LA_INDEX: LazyLock<HashSet<i32>> =
    LazyLock::new(|| WU_LA.iter().filter_map(|key| parse_month_day_key(key)).collect());

pub fn festivals(month: i32, day: i32) -> &'static [Vec<&'static str>] {
    FESTIVAL_INDEX.get(&month_day_key(month, day)).map(Vec::as_slice).unwrap_or(&[])
}

pub fn is_day_san_hui(month: i32, day: i32) -> bool {
    SAN_HUI_INDEX.contains(&month_day_key(month, day))
}

pub fn is_day_san_yuan(month: i32, day: i32) -> bool {
    SAN_YUAN_INDEX.contains(&month_day_key(month, day))
}

pub fn is_day_wu_la(month: i32, day: i32) -> bool {
    WU_LA_INDEX.contains(&month_day_key(month, day))
}
