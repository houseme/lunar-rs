//! Differential-testing support utilities.
//!
//! These helpers define the stable snapshot protocol used by
//! `tests/differential.rs` and the sample reference driver binary.

use crate::Solar;

fn render_nine_star(star: crate::NineStar) -> String {
    format!("{}{}{}", star.number(), star.color(), star.wu_xing())
}

/// Snapshot protocol version for differential-testing tooling.
pub const PROTOCOL_VERSION: &str = "8";

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
    "week_name",
    "week_index",
    "constellation",
    "legal_holiday",
    "legal_holiday_work",
    "solar_nine_star",
    "phenology_day",
    "phase_day",
    "nine_day",
    "hide_heaven_stem_day",
    "dog_day",
    "plum_rain_day",
    "year_ganzhi",
    "month_ganzhi",
    "day_ganzhi",
    "time_ganzhi",
    "lunar_year_month_count",
    "lunar_year_leap_month",
    "lunar_year_sixty_cycle",
    "lunar_year_jupiter_direction",
    "lunar_year_twenty",
    "lunar_year_nine_star",
    "lunar_month",
    "lunar_month_with_leap",
    "lunar_month_day_count",
    "lunar_month_index_in_year",
    "lunar_month_minor_ren",
    "lunar_month_nine_star",
    "lunar_month_sixty_cycle",
    "lunar_month_jupiter_direction",
    "lunar_month_fetus",
    "lunar_hour_name",
    "lunar_hour_index_in_day",
    "lunar_hour_minor_ren",
    "lunar_hour_twelve_star",
    "lunar_hour_nine_star",
    "lunar_six_star",
    "lunar_minor_ren",
    "lunar_twelve_star",
    "lunar_twenty_eight_star",
    "lunar_nine_star",
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
    let legal_holiday = solar.get_legal_holiday();
    let lunar_year = crate::LunarYear::from_year(lunar.get_year());
    let lunar_month = lunar.get_lunar_month();
    let lunar_hour = solar.get_lunar_hour();

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
        ("week_name", solar.get_week().name().to_string()),
        ("week_index", solar.get_week().index().to_string()),
        ("constellation", solar.get_constellation().name().to_string()),
        ("legal_holiday", legal_holiday.as_ref().map_or_else(String::new, |holiday| holiday.get_name())),
        (
            "legal_holiday_work",
            legal_holiday.as_ref().map_or_else(String::new, |holiday| holiday.is_work().to_string()),
        ),
        ("solar_nine_star", render_nine_star(solar.get_nine_star())),
        ("phenology_day", solar.get_phenology_day().map_or_else(String::new, |day| day.to_string())),
        ("phase_day", solar.get_phase_day().to_string()),
        ("nine_day", solar.get_nine_day().map_or_else(String::new, |day| day.to_string())),
        ("hide_heaven_stem_day", solar.get_hide_heaven_stem_day().map_or_else(String::new, |day| day.to_string())),
        ("dog_day", solar.get_dog_day().map_or_else(String::new, |day| day.to_string())),
        ("plum_rain_day", solar.get_plum_rain_day().map_or_else(String::new, |day| day.to_string())),
        ("year_ganzhi", lunar.year_in_gan_zhi()),
        ("month_ganzhi", lunar.month_in_gan_zhi()),
        ("day_ganzhi", lunar.day_in_gan_zhi()),
        ("time_ganzhi", lunar.time_in_gan_zhi()),
        ("lunar_year_month_count", lunar_year.get_month_count().to_string()),
        ("lunar_year_leap_month", lunar_year.get_leap_month().to_string()),
        ("lunar_year_sixty_cycle", lunar_year.get_sixty_cycle().name().to_string()),
        ("lunar_year_jupiter_direction", lunar_year.get_jupiter_direction().name().to_string()),
        ("lunar_year_twenty", lunar_year.get_twenty().name().to_string()),
        ("lunar_year_nine_star", render_nine_star(lunar_year.get_nine_star())),
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
        (
            "lunar_month_minor_ren",
            lunar_month.map_or_else(String::new, |month| month.get_minor_ren().name().to_string()),
        ),
        (
            "lunar_month_nine_star",
            lunar_month.map_or_else(String::new, |month| render_nine_star(month.get_nine_star())),
        ),
        (
            "lunar_month_sixty_cycle",
            lunar_month.map_or_else(String::new, |month| month.get_sixty_cycle().name().to_string()),
        ),
        (
            "lunar_month_jupiter_direction",
            lunar_month.map_or_else(String::new, |month| month.get_jupiter_direction().name().to_string()),
        ),
        (
            "lunar_month_fetus",
            lunar_month.map_or_else(String::new, |month| month.get_fetus().map_or_else(String::new, |fetus| fetus.name().to_string())),
        ),
        ("lunar_hour_name", lunar_hour.get_name()),
        ("lunar_hour_index_in_day", lunar_hour.get_index_in_day().to_string()),
        ("lunar_hour_minor_ren", lunar_hour.get_minor_ren().name().to_string()),
        ("lunar_hour_twelve_star", lunar_hour.get_twelve_star().name().to_string()),
        ("lunar_hour_nine_star", render_nine_star(lunar_hour.get_nine_star())),
        ("lunar_six_star", lunar.get_six_star().name().to_string()),
        ("lunar_minor_ren", lunar.get_minor_ren().name().to_string()),
        ("lunar_twelve_star", lunar.get_twelve_star().name().to_string()),
        ("lunar_twenty_eight_star", lunar.get_twenty_eight_star().name().to_string()),
        ("lunar_nine_star", render_nine_star(lunar.get_nine_star())),
    ]
}
