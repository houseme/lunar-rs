use std::collections::HashSet;

use lunar_rs::Solar;
use lunar_rs::differential_support::{PROTOCOL_VERSION, SOLAR_SNAPSHOT_KEYS, solar_snapshot};

#[test]
fn solar_snapshot_protocol_is_stable() {
    let solar = Solar::from_ymd_hms(2024, 4, 22, 23, 30, 0).unwrap();
    let snapshot = solar_snapshot(solar);

    assert_eq!(snapshot.len(), SOLAR_SNAPSHOT_KEYS.len());

    let keys: Vec<&str> = snapshot.iter().map(|(key, _)| *key).collect();
    assert_eq!(keys, SOLAR_SNAPSHOT_KEYS);

    let unique_keys: HashSet<&str> = keys.iter().copied().collect();
    assert_eq!(unique_keys.len(), keys.len());

    let values = snapshot.into_iter().collect::<std::collections::HashMap<_, _>>();
    assert_eq!(values.get("protocol_version").map(String::as_str), Some(PROTOCOL_VERSION));
    assert_eq!(values.get("calendar").map(String::as_str), Some("solar"));
    assert_eq!(values.get("solar").map(String::as_str), Some("2024-04-22 23:30:00"));
    assert_eq!(values.get("jieqi").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar").map(String::as_str), Some("二〇二四年三月十四"));
    assert_eq!(values.get("year_ganzhi").map(String::as_str), Some("甲辰"));
    assert_eq!(values.get("solar_festival").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar_festival").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar_year_month_count").map(String::as_str), Some("12"));
    assert_eq!(values.get("lunar_year_leap_month").map(String::as_str), Some("0"));
    assert_eq!(values.get("lunar_month").map(String::as_str), Some("3"));
    assert_eq!(values.get("lunar_month_with_leap").map(String::as_str), Some("3"));
}

#[test]
fn solar_snapshot_covers_festival_and_leap_month_cases() {
    let spring = solar_snapshot(Solar::from_ymd_hms(2019, 2, 5, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(spring.get("lunar_festival").map(String::as_str), Some("春节"));
    assert_eq!(spring.get("lunar_festival_index").map(String::as_str), Some("0"));

    let qingming = solar_snapshot(Solar::from_ymd_hms(2024, 4, 4, 12, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(qingming.get("solar_festival").map(String::as_str), Some(""));
    assert_eq!(qingming.get("lunar_festival").map(String::as_str), Some("清明节"));
    assert_eq!(qingming.get("jieqi").map(String::as_str), Some("清明"));

    let leap_month = solar_snapshot(Solar::from_ymd_hms(2020, 5, 23, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(leap_month.get("lunar_year_month_count").map(String::as_str), Some("13"));
    assert_eq!(leap_month.get("lunar_year_leap_month").map(String::as_str), Some("4"));
    assert_eq!(leap_month.get("lunar_month").map(String::as_str), Some("4"));
    assert_eq!(leap_month.get("lunar_month_with_leap").map(String::as_str), Some("-4"));
    assert_eq!(leap_month.get("lunar_month_day_count").map(String::as_str), Some("29"));
}
