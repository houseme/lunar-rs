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
    assert_eq!(values.get("week_name").map(String::as_str), Some("一"));
    assert_eq!(values.get("week_index").map(String::as_str), Some("1"));
    assert_eq!(values.get("constellation").map(String::as_str), Some("金牛"));
    assert_eq!(values.get("legal_holiday").map(String::as_str), Some(""));
    assert_eq!(values.get("legal_holiday_work").map(String::as_str), Some(""));
    assert_eq!(values.get("solar_nine_star").map(String::as_str), Some("五黄土"));
    assert_eq!(values.get("phase_day").map(String::as_str), Some("小望第14天"));
    assert_eq!(values.get("phenology_day").map(String::as_str), Some("萍始生第4天"));
    assert_eq!(values.get("nine_day").map(String::as_str), Some(""));
    assert_eq!(values.get("hide_heaven_stem_day").map(String::as_str), Some("戊土第7天"));
    assert_eq!(values.get("dog_day").map(String::as_str), Some(""));
    assert_eq!(values.get("plum_rain_day").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar").map(String::as_str), Some("二〇二四年三月十四"));
    assert_eq!(values.get("year_ganzhi").map(String::as_str), Some("甲辰"));
    assert_eq!(values.get("solar_festival").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar_festival").map(String::as_str), Some(""));
    assert_eq!(values.get("lunar_year_month_count").map(String::as_str), Some("12"));
    assert_eq!(values.get("lunar_year_leap_month").map(String::as_str), Some("0"));
    assert_eq!(values.get("lunar_year_twenty").map(String::as_str), Some("九运"));
    assert_eq!(values.get("lunar_year_nine_star").map(String::as_str), Some("三碧木"));
    assert_eq!(values.get("lunar_month").map(String::as_str), Some("3"));
    assert_eq!(values.get("lunar_month_with_leap").map(String::as_str), Some("3"));
    assert_eq!(values.get("lunar_month_minor_ren").map(String::as_str), Some("速喜"));
    assert_eq!(values.get("lunar_month_nine_star").map(String::as_str), Some("三碧木"));
    assert_eq!(values.get("lunar_hour_name").map(String::as_str), Some("子时"));
    assert_eq!(values.get("lunar_hour_index_in_day").map(String::as_str), Some("12"));
    assert_eq!(values.get("lunar_hour_minor_ren").map(String::as_str), Some("赤口"));
    assert_eq!(values.get("lunar_six_star").map(String::as_str), Some("佛灭"));
    assert_eq!(values.get("lunar_minor_ren").map(String::as_str), Some("赤口"));
    assert_eq!(values.get("lunar_twelve_star").map(String::as_str), Some("青龙"));
    assert_eq!(values.get("lunar_twenty_eight_star").map(String::as_str), Some("毕"));
    assert_eq!(values.get("lunar_nine_star").map(String::as_str), Some("五黄土"));
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
    assert_eq!(leap_month.get("lunar_year_twenty").map(String::as_str), Some("八运"));
    assert_eq!(leap_month.get("lunar_year_nine_star").map(String::as_str), Some("七赤金"));
    assert_eq!(leap_month.get("lunar_month_minor_ren").map(String::as_str), Some("赤口"));
    assert_eq!(leap_month.get("lunar_month_nine_star").map(String::as_str), Some("五黄土"));

    let holiday = solar_snapshot(Solar::from_ymd_hms(2024, 10, 1, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(holiday.get("legal_holiday").map(String::as_str), Some("国庆节"));
    assert_eq!(holiday.get("legal_holiday_work").map(String::as_str), Some("false"));
    assert_eq!(holiday.get("week_name").map(String::as_str), Some("二"));
    assert_eq!(holiday.get("solar_nine_star").map(String::as_str), Some("五黄土"));

    let nine = solar_snapshot(Solar::from_ymd_hms(2020, 12, 21, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(nine.get("nine_day").map(String::as_str), Some("一九第1天"));

    let dog = solar_snapshot(Solar::from_ymd_hms(2012, 7, 18, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(dog.get("dog_day").map(String::as_str), Some("初伏第1天"));

    let plum = solar_snapshot(Solar::from_ymd_hms(2024, 6, 11, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(plum.get("plum_rain_day").map(String::as_str), Some("入梅第1天"));

    let hide = solar_snapshot(Solar::from_ymd_hms(2024, 12, 4, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(hide.get("hide_heaven_stem_day").map(String::as_str), Some("壬水第16天"));

    let phenology = solar_snapshot(Solar::from_ymd_hms(2021, 12, 21, 0, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(phenology.get("phenology_day").map(String::as_str), Some("蚯蚓结第1天"));

    let lunar_hour = solar_snapshot(Solar::from_ymd_hms(2023, 11, 14, 23, 0, 0).unwrap())
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(lunar_hour.get("lunar_hour_name").map(String::as_str), Some("子时"));
    assert_eq!(lunar_hour.get("lunar_hour_index_in_day").map(String::as_str), Some("12"));
    assert_eq!(lunar_hour.get("lunar_hour_minor_ren").map(String::as_str), Some("小吉"));
}
