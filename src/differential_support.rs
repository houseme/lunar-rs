//! Differential-testing support utilities.
//!
//! These helpers define the stable snapshot protocol used by
//! `tests/differential.rs` and the sample reference driver binary.

use crate::Solar;

/// Snapshot protocol version for differential-testing tooling.
pub const PROTOCOL_VERSION: &str = "2";

/// Stable newline key names emitted by the sample reference driver.
pub const SOLAR_SNAPSHOT_KEYS: &[&str] = &[
    "protocol_version",
    "calendar",
    "solar",
    "solar_full",
    "lunar",
    "lunar_full",
    "solar_festival",
    "solar_festival_index",
    "lunar_festival",
    "lunar_festival_index",
    "jieqi",
    "year_ganzhi",
    "month_ganzhi",
    "day_ganzhi",
    "time_ganzhi",
    "lunar_year_month_count",
    "lunar_year_leap_month",
    "lunar_month",
    "lunar_month_with_leap",
    "lunar_month_day_count",
    "lunar_month_index_in_year",
];

/// Render a stable ordered key-value snapshot for a solar datetime input.
pub fn solar_snapshot(solar: Solar) -> Vec<(&'static str, String)> {
    let lunar = solar.lunar();
    let solar_festival = solar.get_festival().map_or_else(String::new, |festival| festival.get_name());
    let solar_festival_index =
        solar.get_festival().map_or_else(String::new, |festival| festival.get_index().to_string());
    let lunar_festival = lunar.get_festival().map_or_else(String::new, |festival| festival.get_name());
    let lunar_festival_index =
        lunar.get_festival().map_or_else(String::new, |festival| festival.get_index().to_string());
    let lunar_year = crate::LunarYear::from_year(lunar.get_year());
    let lunar_month = lunar.get_lunar_month();

    vec![
        ("protocol_version", PROTOCOL_VERSION.to_string()),
        ("calendar", "solar".to_string()),
        ("solar", solar.to_ymd_hms()),
        ("solar_full", solar.to_full_string()),
        ("lunar", lunar.to_string()),
        ("lunar_full", lunar.to_full_string()),
        ("solar_festival", solar_festival),
        ("solar_festival_index", solar_festival_index),
        ("lunar_festival", lunar_festival),
        ("lunar_festival_index", lunar_festival_index),
        ("jieqi", lunar.jie_qi().to_string()),
        ("year_ganzhi", lunar.year_in_gan_zhi()),
        ("month_ganzhi", lunar.month_in_gan_zhi()),
        ("day_ganzhi", lunar.day_in_gan_zhi()),
        ("time_ganzhi", lunar.time_in_gan_zhi()),
        ("lunar_year_month_count", lunar_year.get_month_count().to_string()),
        ("lunar_year_leap_month", lunar_year.get_leap_month().to_string()),
        ("lunar_month", lunar_month.map_or_else(String::new, |month| month.get_month().to_string())),
        (
            "lunar_month_with_leap",
            lunar_month.map_or_else(String::new, |month| month.get_month_with_leap().to_string()),
        ),
        ("lunar_month_day_count", lunar_month.map_or_else(String::new, |month| month.get_day_count().to_string())),
        (
            "lunar_month_index_in_year",
            lunar_month.map_or_else(String::new, |month| month.get_index_in_year().to_string()),
        ),
    ]
}
