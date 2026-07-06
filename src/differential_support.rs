//! Differential-testing support utilities.
//!
//! These helpers define the stable snapshot protocol used by
//! `tests/differential.rs` and the sample reference driver binary.

use crate::Solar;

/// Snapshot protocol version for differential-testing tooling.
pub const PROTOCOL_VERSION: &str = "1";

/// Stable newline key names emitted by the sample reference driver.
pub const SOLAR_SNAPSHOT_KEYS: &[&str] = &[
    "protocol_version",
    "calendar",
    "solar",
    "solar_full",
    "lunar",
    "lunar_full",
    "jieqi",
    "year_ganzhi",
    "month_ganzhi",
    "day_ganzhi",
    "time_ganzhi",
];

/// Render a stable ordered key-value snapshot for a solar datetime input.
pub fn solar_snapshot(solar: Solar) -> Vec<(&'static str, String)> {
    let lunar = solar.lunar();
    vec![
        ("protocol_version", PROTOCOL_VERSION.to_string()),
        ("calendar", "solar".to_string()),
        ("solar", solar.to_ymd_hms()),
        ("solar_full", solar.to_full_string()),
        ("lunar", lunar.to_string()),
        ("lunar_full", lunar.to_full_string()),
        ("jieqi", lunar.jie_qi().to_string()),
        ("year_ganzhi", lunar.year_in_gan_zhi()),
        ("month_ganzhi", lunar.month_in_gan_zhi()),
        ("day_ganzhi", lunar.day_in_gan_zhi()),
        ("time_ganzhi", lunar.time_in_gan_zhi()),
    ]
}
