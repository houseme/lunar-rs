use std::collections::{HashMap, HashSet};

use lunar_rs::Solar;
use lunar_rs::differential_support::{solar_snapshot, solar_snapshot_keys, PROTOCOL_VERSION};

#[test]
fn solar_snapshot_protocol_is_stable() {
    let solar = Solar::from_ymd_hms(2024, 4, 22, 23, 30, 0).unwrap();
    let snapshot = solar_snapshot(solar);

    // Shape: every field defined in the source-of-truth table is emitted exactly
    // once, in table order.
    let keys: Vec<&'static str> = snapshot.iter().map(|entry| entry.key).collect();
    let expected_keys: Vec<&'static str> = solar_snapshot_keys().collect();
    assert_eq!(keys, expected_keys, "snapshot keys must match the canonical table in order");

    let unique: HashSet<&'static str> = keys.iter().copied().collect();
    assert_eq!(unique.len(), keys.len(), "snapshot keys must be unique");

    // Unambiguous anchors for a known date. Formula-sensitive fields (twelve-star,
    // nine-star, etc.) are exercised by the differential suite and by the
    // festival/leap-month case test below rather than hard-coded here.
    let value = |key: &str| -> &str {
        snapshot.iter().find(|e| e.key == key).unwrap_or_else(|| panic!("missing key `{key}`")).value.as_str()
    };
    assert_eq!(value("protocol_version"), PROTOCOL_VERSION);
    assert_eq!(value("calendar"), "solar");
    assert_eq!(value("solar"), "2024-04-22 23:30:00");
    assert_eq!(value("lunar"), "二〇二四年三月十四");
    assert_eq!(value("year_ganzhi"), "甲辰");
}

#[test]
fn solar_snapshot_covers_festival_and_leap_month_cases() {
    let by_key = |solar: Solar| -> HashMap<&'static str, String> {
        solar_snapshot(solar)
            .into_iter()
            .map(|entry| (entry.key, entry.value.as_str().to_string()))
            .collect()
    };

    let spring = by_key(Solar::from_ymd_hms(2019, 2, 5, 0, 0, 0).unwrap());
    assert_eq!(spring.get("lunar_festival").map(String::as_str), Some("春节"));
    assert_eq!(spring.get("lunar_festival_index").map(String::as_str), Some("0"));

    let qingming = by_key(Solar::from_ymd_hms(2024, 4, 4, 12, 0, 0).unwrap());
    assert_eq!(qingming.get("solar_festival").map(String::as_str), Some(""));
    assert_eq!(qingming.get("lunar_festival").map(String::as_str), Some("清明节"));
    assert_eq!(qingming.get("jieqi").map(String::as_str), Some("清明"));

    let leap_month = by_key(Solar::from_ymd_hms(2020, 5, 23, 0, 0, 0).unwrap());
    assert_eq!(leap_month.get("lunar_year_month_count").map(String::as_str), Some("13"));
    assert_eq!(leap_month.get("lunar_year_leap_month").map(String::as_str), Some("4"));
    assert_eq!(leap_month.get("lunar_year_sixty_cycle").map(String::as_str), Some("庚子"));
    assert_eq!(leap_month.get("lunar_year_jupiter_direction").map(String::as_str), Some("北"));
    assert_eq!(leap_month.get("lunar_month").map(String::as_str), Some("4"));
    assert_eq!(leap_month.get("lunar_month_with_leap").map(String::as_str), Some("-4"));
    assert_eq!(leap_month.get("lunar_month_day_count").map(String::as_str), Some("29"));
    assert_eq!(leap_month.get("lunar_year_twenty").map(String::as_str), Some("八运"));
    assert_eq!(leap_month.get("lunar_year_nine_star").map(String::as_str), Some("七赤金"));
    assert_eq!(leap_month.get("lunar_month_minor_ren").map(String::as_str), Some("赤口"));
    assert_eq!(leap_month.get("lunar_month_nine_star").map(String::as_str), Some("五黄土"));
    assert_eq!(leap_month.get("lunar_month_sixty_cycle").map(String::as_str), Some("辛巳"));
    assert_eq!(leap_month.get("lunar_month_jupiter_direction").map(String::as_str), Some("东南"));
    assert_eq!(leap_month.get("lunar_month_fetus").map(String::as_str), Some(""));

    let holiday = by_key(Solar::from_ymd_hms(2024, 10, 1, 0, 0, 0).unwrap());
    assert_eq!(holiday.get("legal_holiday").map(String::as_str), Some("国庆节"));
    assert_eq!(holiday.get("legal_holiday_work").map(String::as_str), Some("false"));
    assert_eq!(holiday.get("week_name").map(String::as_str), Some("二"));
    assert_eq!(holiday.get("solar_nine_star").map(String::as_str), Some("五黄土"));

    let nine = by_key(Solar::from_ymd_hms(2020, 12, 21, 0, 0, 0).unwrap());
    assert_eq!(nine.get("nine_day").map(String::as_str), Some("一九第1天"));

    let dog = by_key(Solar::from_ymd_hms(2012, 7, 18, 0, 0, 0).unwrap());
    assert_eq!(dog.get("dog_day").map(String::as_str), Some("初伏第1天"));

    let plum = by_key(Solar::from_ymd_hms(2024, 6, 11, 0, 0, 0).unwrap());
    assert_eq!(plum.get("plum_rain_day").map(String::as_str), Some("入梅第1天"));

    let hide = by_key(Solar::from_ymd_hms(2024, 12, 4, 0, 0, 0).unwrap());
    assert_eq!(hide.get("hide_heaven_stem_day").map(String::as_str), Some("壬水第16天"));

    let phenology = by_key(Solar::from_ymd_hms(2021, 12, 21, 0, 0, 0).unwrap());
    assert_eq!(phenology.get("phenology_day").map(String::as_str), Some("蚯蚓结第1天"));

    // Lunar-hour twelve-star uses the corrected 建除 formula (aligned with
    // `Lunar::get_twelve_star`); the value below reflects the post-fix output.
    let lunar_hour = by_key(Solar::from_ymd_hms(2023, 11, 14, 23, 0, 0).unwrap());
    assert_eq!(lunar_hour.get("lunar_hour_name").map(String::as_str), Some("子时"));
    assert_eq!(lunar_hour.get("lunar_hour_index_in_day").map(String::as_str), Some("12"));
    assert_eq!(lunar_hour.get("lunar_hour_minor_ren").map(String::as_str), Some("小吉"));
    assert_eq!(lunar_hour.get("lunar_hour_twelve_star").map(String::as_str), Some("金匮"));
    assert_eq!(lunar_hour.get("lunar_hour_nine_star").map(String::as_str), Some("九紫火"));
}
