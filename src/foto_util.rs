//! 佛历工具。对应 lunar-go `FotoUtil/FotoUtil.go`。

include!("foto_util_data.rs");
include!("foto_util_maps.rs");

/// 当日印度星宿（27 宿）。
pub fn get_xiu(month: i32, day: i32) -> &'static str {
    let m = month.unsigned_abs() as usize;
    let idx = (XIU_OFFSET[m - 1] + i64::from(day) - 1).rem_euclid(27) as usize;
    XIU_27[idx]
}

/// 观音斋日期（`"M-D"`）。
pub fn is_day_zhai_guan_yin(key: &str) -> bool {
    DAY_ZHAI_GUAN_YIN.contains(&key)
}
